use clap::builder::NonEmptyStringValueParser;
use std::str::FromStr;

const KEY_VALUE_ERROR_MSG: &str = "must have the form key=value";

pub fn non_empty() -> NonEmptyStringValueParser {
    NonEmptyStringValueParser::new()
}

#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl FromStr for KeyValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((key, value)) = s.split_once("=") else {
            return Err(KEY_VALUE_ERROR_MSG.into());
        };

        if key.is_empty() || value.is_empty() {
            return Err(KEY_VALUE_ERROR_MSG.into());
        }

        Ok(KeyValue {
            key: String::from(key),
            value: String::from(value),
        })
    }
}
