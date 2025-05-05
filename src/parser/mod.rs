pub mod parsers;

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub event_desc: String,
    pub data: Vec<String>,
}

pub trait DescriptionParser: Sync + Send {
    fn parse_description(&self, data: Vec<u8>) -> ParseResult;
}
