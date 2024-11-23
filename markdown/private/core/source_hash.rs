use clap::Parser;
use markdown::arg_validators;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::{MetadataMap, SourceHash};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let dep_metadata: MetadataMap = from_json(&read_to_string(args.deps_metadata_file)?)?;
    let dep_hashes = BTreeMap::from_iter(
        dep_metadata
            .data()
            .iter()
            .map(|(target, metadata)| (target, metadata.source_hash())),
    );

    let src = read_to_string(args.src_file)?;

    let mut hash_input = vec![String::from("{")];
    hash_input.extend(
        dep_hashes
            .iter()
            .map(|(target, source_hash)| format!("{target} {source_hash}")),
    );
    hash_input.push(String::from("}"));
    hash_input.push(src);

    let hash_output = format!("{:x}", Sha256::digest(hash_input.join("\n")));

    SourceHash::build(&hash_output)?.write_json(args.out_file)
}
