pub mod parsers;

pub trait DescriptionParser: Sync + Send {
    fn parse_description(&self, data: Vec<u8>) -> String;
}
