use clap::Parser;
use markdown::args;
use markdown::json::from_json;
use markdown::metadata::{InputMetadataBuilder, MetadataMap};
use std::error::Error;
use std::fs::{read_to_string, write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = args::non_empty())]
    title: String,

    #[arg(value_parser = args::non_empty())]
    author: String,

    #[arg(long)]
    #[arg(value_parser = args::non_empty())]
    date: Option<String>,

    #[arg(value_parser = args::non_empty())]
    metadata_file: String,

    #[arg(value_parser = args::non_empty())]
    out_file: String,

    #[arg(long = "dep")]
    #[arg(value_parser = args::non_empty())]
    deps: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut builder = InputMetadataBuilder::default();
    builder.title(args.title);
    builder.authors([args.author.clone()]);

    if let Some(date) = args.date {
        builder.date(date);
    }

    let main_metadata = builder.build()?;

    let mut output = vec![String::from("---")];

    output.extend(
        serde_yaml::to_string(&main_metadata)?
            .trim()
            .split("\n")
            .map(String::from),
    );
    output.push(String::from("---\n"));

    let metadata: MetadataMap = from_json(&read_to_string(args.metadata_file)?)?;

    for target in args.deps {
        let Some(m) = metadata.data().get(&target) else {
            return Err(format!("target '{}' not found", target).into());
        };
        output.push(format!(
            "::: nospellcheck

# {}",
            m.title().cloned().unwrap_or_default()
        ));
        let mut tagline = Vec::new();
        if let Some(author) = m.authors().first() {
            if *author != args.author {
                tagline.push(author.clone());
            }
        }
        if let Some(date) = m.date() {
            tagline.push(date.clone());
        }
        if !tagline.is_empty() {
            output.push(format!(
                "
**{}**

::: collectionseparator
&nbsp;
:::",
                tagline.join(", ")
            ));
        }
        output.push(format!(
            "
:::

!include //{target}
"
        ));
    }

    write(args.out_file, output.join("\n"))?;
    Ok(())
}
