use crate::parser::{DescriptionParser, ParseResult};
pub struct EvEventTagParser;
impl DescriptionParser for EvEventTagParser {
    fn parse_description(&self, data: Vec<u8>) -> ParseResult {
        let length =
            u32::from_le_bytes(data[4..8].try_into().expect("Failed to extract length")) as usize;
        let description_bytes = &data[8..8 + length];
        let event_desc = String::from_utf8(description_bytes.to_vec())
            .expect("Invalid UTF-8 text")
            .replace('\0', "");
        ParseResult {event_desc, data: vec![]}
    }
}
