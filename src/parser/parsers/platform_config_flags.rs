use crate::parser::DescriptionParser;
use crate::EventDetails;
use anyhow::{Error, Result};
use serde::Serialize;

pub struct EvPlatformConfigFlagsParser;

#[derive(Debug, Clone, Serialize)]
pub struct EventDetail {
    pub name: String,
    pub value: Option<String>,
}

impl DescriptionParser for EvPlatformConfigFlagsParser {
    fn parse_description(&self, data: Vec<u8>) -> Result<EventDetails, Error> {
        let mut result_data = EventDetails::empty();
        let raw = String::from_utf8(data)
            .unwrap_or_default()
            .replace('\u{10}', " ");
        let map = parse_kernel_parameters(raw.clone());
        let cleaned = raw.chars().filter(|c| !c.is_control()).collect::<String>();
        result_data.string = Some(cleaned);
        result_data.data = Some(map);
        Ok(result_data)
    }
}

fn parse_kernel_parameters(kernel_parameters: String) -> Vec<String> {
    let parameters = kernel_parameters
        .split(&[' ', '\n', '\r', '\0', '\u{10}'])
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|item| {
            if item.is_empty() {
                return None;
            }

            let it = item.split_once('=');

            match it {
                Some((k, v)) => {
                    let detail = EventDetail {
                        name: k.into(),
                        value: Some(v.into()),
                    };
                    let json = serde_json::to_string(&detail).expect("Failed to encode json");
                    Some(json)
                }
                None => {
                    let detail = EventDetail {
                        name: item.to_string(),
                        value: None,
                    };
                    let json = serde_json::to_string(&detail).expect("Failed to encode json");
                    Some(json)
                }
            }
        })
        .collect();

    parameters
}
