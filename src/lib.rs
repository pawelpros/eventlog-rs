use anyhow::{anyhow, Result};
use core::fmt;
use sha2::Digest;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::u32;

mod bios_eventlog;
mod enums;
pub mod rtmr;
pub mod tcg_algorithm;
pub mod tcg_enum;

pub use bios_eventlog::BiosEventlog;

mod parser;
pub mod read;
mod utils;

use crate::tcg_algorithm::TcgAlgorithm;
use crate::tcg_enum::TcgEventType;

#[derive(Clone)]
pub struct Eventlog {
    pub log: Vec<EventlogEntry>,
}

impl fmt::Display for Eventlog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parsed_el = String::default();
        for event_entry in self.log.clone() {
            parsed_el = format!(
                "{}\nEvent Entry:\n\tRTMR: {}\n\tEvent Type id: {}\n\tEvent Type: {}\n\tDigest Algorithm: {}\n\tDigest: {}\n\tEvent Desc: {}\n\tEvent Desc HEX: {}\n",
                parsed_el,
                event_entry.rtmr,
                format!("0x{:08X}", event_entry.event_type as u32),
                event_entry.event_type,
                event_entry.digests[0].algorithm,
                hex::encode(event_entry.digests[0].digest.clone()),
                event_entry.event_desc.clone(),
                event_entry.event_desc_hex.clone()
            );
        }

        write!(f, "{parsed_el}")
    }
}

#[derive(Debug, Clone)]
pub struct EventlogEntry {
    pub rtmr: u32,
    pub event_type: TcgEventType,
    pub digests: Vec<ElDigest>,
    pub event_desc_hex: String,
    pub event_desc: String,
}

#[derive(Debug, Clone)]
pub struct ElDigest {
    pub algorithm: TcgAlgorithm,
    pub digest: Vec<u8>,
}

impl TryFrom<Vec<u8>> for Eventlog {
    type Error = anyhow::Error;

    fn try_from(data: Vec<u8>) -> Result<Self> {
        let mut index = 0;
        let mut event_log = Vec::new();
        let mut digest_size_map = HashMap::new();

        while index < data.len() {
            let entry_opt;
            (entry_opt, index) = parse_eventlog_entry(&data, index, &mut digest_size_map)?;
            if let Some(entry) = entry_opt {
                event_log.push(entry);
            } else if index == 0 {
                break;
            }
        }

        Ok(Eventlog { log: event_log })
    }
}

fn parse_eventlog_entry(
    data: &[u8],
    mut index: usize,
    digest_size_map: &mut HashMap<TcgAlgorithm, u16>,
) -> Result<(Option<EventlogEntry>, usize)> {
    let (stop_flag, _new_index) = utils::read_long(data, index)?;
    if stop_flag == 0xFFFFFFFFFFFFFFFF || stop_flag == 0x0000000000000000 {
        return Ok((None, 0));
    }

    let target_measurement_registry;
    (target_measurement_registry, index) = utils::read_int(data, index)?;

    let event_type_num;
    (event_type_num, index) = utils::read_int(data, index)?;

    let event_type = TcgEventType::try_from(event_type_num)
        .map_err(|_| anyhow!("Unknown event type detected: {:#x}", event_type_num))?;

    if event_type == TcgEventType::EvNoAction {
        index = parse_digest_sizes(data, index, digest_size_map)?;
        return Ok((None, index));
    }

    let digests;
    (digests, index) = parse_digests(data, index, digest_size_map)?;

    let event_desc_size;
    (event_desc_size, index) = utils::read_int(data, index)?;
    let event_desc_raw = data[index..(index + event_desc_size as usize)].to_vec();
    index += event_desc_size as usize;

    let event_desc_hex = hex::encode(&event_desc_raw);
    let event_desc = event_type.get_parser().parse_description(event_desc_raw);

    Ok((
        Some(EventlogEntry {
            rtmr: target_measurement_registry,
            event_type,
            digests,
            event_desc_hex,
            event_desc,
        }),
        index,
    ))
}

fn parse_digest_sizes(
    data: &[u8],
    mut index: usize,
    digest_size_map: &mut HashMap<TcgAlgorithm, u16>,
) -> Result<usize> {
    index += 48;
    let algo_number;
    (algo_number, index) = utils::read_int(data, index)?;

    for _ in 0..algo_number {
        let algo_id;
        (algo_id, index) = utils::read_short(data, index)?;
        let size;
        (size, index) = utils::read_short(data, index)?;

        let algorithm = TcgAlgorithm::try_from(algo_id as u32)
            .map_err(|_| anyhow!("Unknown algorithm type detected: {:x}", algo_id))?;

        digest_size_map.insert(algorithm, size);
    }

    let vendor_size = data[index] as usize;
    index += vendor_size + 1;
    Ok(index)
}

fn parse_digests(
    data: &[u8],
    mut index: usize,
    digest_size_map: &HashMap<TcgAlgorithm, u16>,
) -> Result<(Vec<ElDigest>, usize)> {
    let digest_count;
    (digest_count, index) = utils::read_int(data, index)?;

    let mut digests = Vec::new();
    for _ in 0..digest_count {
        let algo_id;
        (algo_id, index) = utils::read_short(data, index)?;

        let algorithm = TcgAlgorithm::try_from(algo_id as u32)
            .map_err(|_| anyhow!("Unknown algorithm type detected: {:x}", algo_id))?;

        let size = *digest_size_map
            .get(&algorithm)
            .ok_or_else(|| anyhow!("Missing digest size for algorithm: {:x}", algo_id))?
            as usize;

        let digest = data[index..index + size].to_vec();
        index += size;

        digests.push(ElDigest { algorithm, digest });
    }

    Ok((digests, index))
}
