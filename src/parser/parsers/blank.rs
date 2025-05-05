use crate::parser::{DescriptionParser, ParseResult};
pub struct EvBlankParser;
impl DescriptionParser for EvBlankParser {
    fn parse_description(&self, _data: Vec<u8>) -> ParseResult {
        ParseResult {event_desc: String::default(), data: vec![]}
    }
}
