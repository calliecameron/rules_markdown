use clap::Parser;
use markdown::arg_validators;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::{MetadataMap, SourceHash};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = arg_validators::non_empty())]
    src_file: String,
    #[arg(value_parser = arg_validators::non_empty())]
    deps_metadata_file: String,
    #[arg(value_parser = arg_validators::non_empty())]
    out_file: String,
}

#[derive(Serialize)]
struct DepHashes<'a>(HashMap<&'a String, &'a str>);

impl JsonSerializable for DepHashes<'_> {}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let dep_metadata: MetadataMap = from_json(&read_to_string(args.deps_metadata_file)?)?;
    let dep_hashes: DepHashes = DepHashes(HashMap::from_iter(
        dep_metadata
            .data()
            .iter()
            .map(|(target, metadata)| (target, metadata.source_hash())),
    ));

    let src = read_to_string(args.src_file)?;

    let hash_input = dep_hashes.to_json()? + src.as_str();
    let hash_output = format!("{:x}", md5::compute(hash_input));

    SourceHash::build(&hash_output)?.write_json(args.out_file)
}
