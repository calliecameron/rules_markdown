use clap::Parser;
use markdown::args;
use markdown::json::{JsonSerializable, from_json};
use markdown::metadata::OutputMetadata;
use serde::Serialize;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = args::non_empty())]
    in_file: String,

    #[arg(value_parser = args::non_empty())]
    out_file: String,
}

#[derive(Serialize)]
struct ShunnMetadata {
    short_title: String,
    author_lastname: String,
    contact_name: String,
    contact_address: String,
    contact_city_state_zip: String,
    contact_phone: String,
    contact_email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    author: Vec<String>,
}

impl JsonSerializable for ShunnMetadata {}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let metadata: OutputMetadata = from_json(&read_to_string(args.in_file)?)?;

    let title = metadata
        .title()
        .cloned()
        .unwrap_or(String::from("[Untitled]"));
    let author = metadata
        .authors()
        .first()
        .cloned()
        .unwrap_or(String::from("[Unknown]"));

    let out = ShunnMetadata {
        short_title: title.clone(),
        author_lastname: author.split_ascii_whitespace().last().unwrap().to_string(),
        contact_name: author.clone(),
        contact_address: String::from("`\\n`{=tex}"),
        contact_city_state_zip: String::from("`\\n`{=tex}"),
        contact_phone: String::from("`\\n`{=tex}"),
        contact_email: String::from("`\\n`{=tex}"),
        title: if metadata.title().is_none() {
            Some(title)
        } else {
            None
        },
        author: if metadata.authors().is_empty() {
            vec![author]
        } else {
            Vec::new()
        },
    };

    out.write_json(args.out_file)
}
