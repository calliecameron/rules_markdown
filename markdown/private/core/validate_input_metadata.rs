use clap::Parser;
use markdown::args;
use markdown::json::{from_json, JsonSerializable};
use markdown::metadata::InputMetadata;
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let metadata: InputMetadata = from_json(&read_to_string(args.in_file)?)?;
    metadata.write_json(args.out_file)
}
