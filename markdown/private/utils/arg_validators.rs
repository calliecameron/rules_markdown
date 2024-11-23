use clap::builder::NonEmptyStringValueParser;

pub fn non_empty() -> NonEmptyStringValueParser {
    NonEmptyStringValueParser::new()
}
