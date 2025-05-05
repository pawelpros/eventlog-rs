use crate::parser::{DescriptionParser, ParseResult};

pub struct EvSimpleParser;

impl DescriptionParser for EvSimpleParser {
    fn parse_description(&self, data: Vec<u8>) -> ParseResult {
        let event_desc = String::from_utf8(data).unwrap_or(String::default());
        ParseResult {event_desc, data: vec![]}
    }
}
