use crate::tcg_enum::TcgAlgorithm;
use crate::CcEventLog;
use anyhow::bail;
use anyhow::*;
use sha2::{Digest, Sha256, Sha384, Sha512};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::result::Result::Ok;

const RTMR_LENGTH_BY_BYTES: usize = 48;
const CHECK_RTMR_LIMIT: usize = 3;

#[derive(Debug, Clone)]
pub struct Rtmr {
    pub data: [Vec<u8>; 4],
}

impl Rtmr {
    pub fn integrity_check(&self, rtmr_from_quote: [Vec<u8>; 4]) -> Result<()> {
        for index in 0..CHECK_RTMR_LIMIT {
            let ccel_value = &self.data[index];
            let quote_value = &rtmr_from_quote[index];
            if ccel_value != quote_value {
                bail!(
                    "CCEL eventlog does not pass RTMR [{}] check. CCEL value: {}, Quote value: {}",
                    index,
                    hex::encode(ccel_value),
                    hex::encode(quote_value)
                );
            }
        }

        Ok(())
    }
}

impl TryFrom<CcEventLog> for Rtmr {
    type Error = anyhow::Error;

    fn try_from(data: CcEventLog) -> anyhow::Result<Self> {
        let mut result: HashMap<u32, [u8; RTMR_LENGTH_BY_BYTES]> = HashMap::new();

        for entry in data.log.into_iter() {
            let digest = &entry.digests[0].digest;

            let mr_value = result
                .entry(entry.index)
                .or_insert([0u8; RTMR_LENGTH_BY_BYTES]);

            let hash = accumulate_hash(
                entry.digests[0].alg,
                mr_value.clone().to_vec(),
                digest.as_slice(),
            )?;

            mr_value.copy_from_slice(&hash);
        }

        let mut data: [Vec<u8>; 4] = [
            vec![0u8; RTMR_LENGTH_BY_BYTES],
            vec![0u8; RTMR_LENGTH_BY_BYTES],
            vec![0u8; RTMR_LENGTH_BY_BYTES],
            vec![0u8; RTMR_LENGTH_BY_BYTES],
        ];

        for index in 1..5 {
            if let Some(value) = result.get(&index) {
                data[index as usize - 1] = value.to_vec();
            }
        }

        Ok(Rtmr { data })
    }
}

fn accumulate_hash(alg: TcgAlgorithm, materials: Vec<u8>, digest: &[u8]) -> Result<Vec<u8>> {
    let result = match alg {
        TcgAlgorithm::Sha256 => hash_with::<Sha256>(&materials, digest),
        TcgAlgorithm::Sha384 => hash_with::<Sha384>(&materials, digest),
        TcgAlgorithm::Sha512 => hash_with::<Sha512>(&materials, digest),
        _ => bail!("Unsupported Hash algorithm {:?}", alg),
    };

    Ok(result)
}

fn hash_with<D: Digest + Default>(materials: &[u8], digest: &[u8]) -> Vec<u8> {
    let mut hasher = D::default();
    hasher.update(materials);
    hasher.update(digest);
    hasher.finalize().to_vec()
}

