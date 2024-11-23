use clap::Parser;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::{MetadataMap, SourceHash};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    src_file: String,
    deps_metadata_file: String,
    metadata_out_file: String,
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

    SourceHash::build(&hash_output)?.write_json(args.metadata_out_file)
}
