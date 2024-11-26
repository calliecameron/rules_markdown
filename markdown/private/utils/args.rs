use clap::builder::NonEmptyStringValueParser;
use std::str::FromStr;

pub fn non_empty() -> NonEmptyStringValueParser {
    NonEmptyStringValueParser::new()
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyValue {
    key: String,
    value: String,
}

impl KeyValue {
    pub fn build(key: &str, value: &str) -> Result<KeyValue, String> {
        if key.is_empty() || value.is_empty() {
            return Err(format!(
                "must have the form key=value where both parts are non-empty; got '{key}' '{value}'"
            ));
        }
        Ok(KeyValue {
            key: String::from(key),
            value: String::from(value),
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_tuple(self) -> (String, String) {
        (self.key, self.value)
    }
}

impl FromStr for KeyValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut key = String::new();
        let mut value = String::new();
        let mut target = &mut key;
        let mut seen_separator = false;

        let mut i = s.chars().peekable();
        while let Some(c) = i.next() {
            if c == '=' && !seen_separator {
                target = &mut value;
                seen_separator = true;
            } else if c == '\\' {
                if let Some(&c2) = i.peek() {
                    if c2 == '\\' || c2 == '=' {
                        target.push(c2);
                        i.next();
                    } else {
                        return Err(format!("unknown escape sequence '\\{c2}' in '{s}'"));
                    }
                } else {
                    return Err(format!("incomplete escape sequence at the end of '{s}'"));
                }
            } else {
                target.push(c);
            }
        }

        KeyValue::build(&key, &value)
    }
}

#[cfg(test)]
mod test_args {
    use super::{FromStr, KeyValue};

    #[test]
    fn test_key_value() {
        assert!(KeyValue::from_str("").is_err());
        assert!(KeyValue::from_str("foo").is_err());
        assert!(KeyValue::from_str("=").is_err());
        assert!(KeyValue::from_str("foo=").is_err());
        assert!(KeyValue::from_str("=foo").is_err());
        assert!(KeyValue::from_str("\\foo=bar").is_err());
        assert!(KeyValue::from_str("foo\\=bar").is_err());
        assert!(KeyValue::from_str("foo=bar\\").is_err());

        assert_eq!(
            KeyValue::from_str("foo=bar").unwrap(),
            KeyValue::build("foo", "bar").unwrap()
        );
        assert_eq!(
            KeyValue::from_str("foo=bar=baz").unwrap(),
            KeyValue::build("foo", "bar=baz").unwrap()
        );
        assert_eq!(
            KeyValue::from_str("foo\\=bar=baz").unwrap(),
            KeyValue::build("foo=bar", "baz").unwrap()
        );
        assert_eq!(
            KeyValue::from_str("foo\\\\=bar\\=baz").unwrap(),
            KeyValue::build("foo\\", "bar=baz").unwrap()
        );
    }
}
