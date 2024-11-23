use clap::Parser;
use markdown::arg_validators;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::{MetadataMap, OutputMetadata};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone)]
struct MetadataFile {
    target: String,
    path: String,
}

impl FromStr for MetadataFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((target, path)) = s.split_once("=") else {
            return Err("must have the form target=path".into());
        };

        if target.is_empty() || path.is_empty() {
            return Err("must have the form target=path".into());
        }

        Ok(MetadataFile {
            target: String::from(target),
            path: String::from(path),
        })
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = arg_validators::non_empty())]
    out_file: String,

    #[arg(long = "metadata-file")]
    metadata_files: Vec<MetadataFile>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let out = args
        .metadata_files
        .into_iter()
        .map(|m| Ok((m.target, from_json(&read_to_string(m.path)?)?)))
        .collect::<Result<BTreeMap<String, OutputMetadata>, Box<dyn Error>>>()?;

    MetadataMap::build(out)?.write_json(args.out_file)
}
