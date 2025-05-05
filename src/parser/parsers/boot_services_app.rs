use byteorder::{ByteOrder, LittleEndian};
use crate::parser::{DescriptionParser, ParseResult};
pub struct EvBootServicesAppParser;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EventDetail {
    pub efi_type: u8,
    pub efi_sub_type: u8,
    pub data: String,
}

impl DescriptionParser for EvBootServicesAppParser {
    fn parse_description(&self, data: Vec<u8>) -> ParseResult {
        // ImageLocationInMemory + ImageLengthInMemory + ImageLinkTimeAddress (24) and 8 for length
        let length_of_device_path = u64::from_le_bytes(data[24..32].try_into().unwrap());

        if length_of_device_path == 0 {
            return ParseResult {event_desc:String::default(), data: vec![]};
        }

        // Calculate the start of the device path and ensure data length
        let device_path_start = 32;
        let device_path_end_header = 4;
        let device_path_end =
            device_path_start + (length_of_device_path - device_path_end_header) as usize;

        if data.len() <= device_path_end {
            return ParseResult {event_desc:String::default(), data: vec![]};
        }

        let device_path_bytes = &data[device_path_start..device_path_end];
        let data:Vec<String> = Vec::new();
        let result_data = ParseResult {event_desc:String::default(), data};
        get_nested_data(&device_path_bytes, result_data)
    }
}

fn get_nested_data(device_path_bytes: &[u8], mut result: ParseResult) -> ParseResult {
    let efi_type = u8::from_le_bytes(device_path_bytes[0..1].try_into().unwrap());
    let efi_sub_type = u8::from_le_bytes(device_path_bytes[1..2].try_into().unwrap());
    let efi_length = u16::from_le_bytes(device_path_bytes[2..4].try_into().unwrap());
    let vendor_data_raw = &device_path_bytes[4..efi_length as usize];

    let device_path = &device_path_bytes[efi_length as usize..];

    let detail = EventDetail { efi_type, efi_sub_type, data: recover_string(vendor_data_raw)};
    let json = serde_json::to_string(&detail).expect("Cannot serialize JSON");
    result.data.push(json);
    result.event_desc = recover_string(vendor_data_raw);

    // https://uefi.org/specs/UEFI/2.10/10_Protocols_Device_Path_Protocol.html#generic-device-path-node-structure
    if device_path.len() == 0 {
        return result
    }

    get_nested_data(&device_path, result)
}

fn recover_string(vendor_data_raw: &[u8]) -> String {
    if !is_utf16_encoded_text(vendor_data_raw) {
        return hex::encode(vendor_data_raw)
    };

    let device_path: Vec<u16> = vendor_data_raw
        .chunks(2)
        .map(|chunk| LittleEndian::read_u16(chunk))
        .take_while(|&x| x != 0)
        .collect();

    String::from_utf16(&device_path).expect("Could not convert data to string")
}

pub fn is_utf16_encoded_text(data: &[u8]) -> bool {
    if data.len() < 2 || data.len() % 2 != 0 {
        return false;
    }

    let utf16: Vec<u16> = data
        .chunks(2)
        .map(|chunk| LittleEndian::read_u16(chunk))
        .take_while(|&x| x != 0)
        .collect();

    if let Ok(decoded) = String::from_utf16(&utf16) {
        let printable_chars = decoded
            .chars()
            .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
            .count();

        let ratio = printable_chars as f32 / decoded.len().max(1) as f32;
        return ratio > 0.9;
    }

    false
}
