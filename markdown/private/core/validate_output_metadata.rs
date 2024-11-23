use clap::Parser;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::OutputMetadata;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    in_file: String,
    out_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let metadata: OutputMetadata = from_json(&read_to_string(args.in_file)?)?;
    metadata.write_json(args.out_file)
}
