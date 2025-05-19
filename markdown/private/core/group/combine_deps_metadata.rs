use clap::Parser;
use markdown::args::{KeyValue, non_empty};
use markdown::json::{JsonSerializable, from_json};
use markdown::metadata::{MetadataMap, OutputMetadata};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = non_empty())]
    out_file: String,

    #[arg(long = "metadata-file")]
    metadata_files: Vec<KeyValue>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let out = args
        .metadata_files
        .into_iter()
        .map(|kv| {
            Ok((
                String::from(kv.key()),
                from_json(&read_to_string(kv.value())?)?,
            ))
        })
        .collect::<Result<BTreeMap<String, OutputMetadata>, Box<dyn Error>>>()?;

    MetadataMap::build(out)?.write_json(args.out_file)
}
