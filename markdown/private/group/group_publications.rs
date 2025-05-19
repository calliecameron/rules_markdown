use clap::Parser;
use markdown::args;
use markdown::json::{JsonSerializable, from_json};
use markdown::metadata::{MetadataMap, OutputMetadata};
use markdown::publications::{Publication, State};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::fs::{read_to_string, write};

fn capitalise(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if let Some(c) = chars.first_mut() {
        c.make_ascii_uppercase();
    }
    String::from_iter(chars)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = args::non_empty())]
    metadata_file: String,

    #[arg(value_parser = args::non_empty())]
    out_file: String,
}

fn generate_header(venues: &BTreeSet<&str>) -> Vec<String> {
    let mut out = vec![
        String::from("<thead>"),
        String::from("<tr>"),
        String::from("<th title=\"Target\">Target</th>"),
        String::from("<th title=\"Title\">Title</th>"),
        String::from("<th title=\"Wordcount\">Wordcount</th>"),
        String::from("<th style=\"border-right: 3px solid\" title=\"Notes\">Notes</th>"),
    ];
    out.extend(venues.iter().map(|v| {
        format!(
            "<th title=\"{}\">{}</th>",
            html_escape::encode_double_quoted_attribute(v),
            html_escape::encode_text(v)
        )
    }));
    out.extend([String::from("</tr>"), String::from("</thead>")]);
    out
}

fn generate_row(target: &str, venues: &BTreeSet<&str>, metadata: &OutputMetadata) -> Vec<String> {
    let ps: HashMap<&str, &Publication> = HashMap::from_iter(
        metadata
            .publications()
            .publications()
            .iter()
            .map(|p| (p.venue(), p)),
    );

    let class_attr = {
        if let Some(state) = metadata.publications().highest_active_state() {
            state.to_string()
        } else {
            String::new()
        }
    };

    let mut out = vec![
        String::from("<tr>"),
        format!(
            "<td class=\"{class_attr}\" title=\"{}\"><a href=\"#{}\">{}</a></td>",
            html_escape::encode_double_quoted_attribute(target),
            html_escape::encode_double_quoted_attribute(target),
            html_escape::encode_text(target)
        ),
        format!(
            "<td title=\"{}\">{}</td>",
            html_escape::encode_double_quoted_attribute(metadata.title().unwrap_or(&String::new())),
            html_escape::encode_text(metadata.title().unwrap_or(&String::new())),
        ),
        format!(
            "<td title=\"{}\">{}</td>",
            html_escape::encode_double_quoted_attribute(&metadata.wordcount().to_string()),
            html_escape::encode_text(&metadata.wordcount().to_string())
        ),
        format!(
            "<td style=\"border-right: 3px solid\" title=\"{}\">{}</td>",
            html_escape::encode_double_quoted_attribute(metadata.notes().unwrap_or(&String::new())),
            html_escape::encode_text(metadata.notes().unwrap_or(&String::new()))
        ),
    ];

    for v in venues {
        out.push(if let Some(p) = ps.get(v) {
            generate_cell(target, p)
        } else {
            String::from("<td></td>")
        });
    }

    out.push(String::from("</tr>"));
    out
}

fn generate_cell(target: &str, p: &Publication) -> String {
    let content = Vec::from_iter(p.dates().iter().map(|d| {
        format!(
            "{} {}",
            d.date.format("%Y-%m-%d"),
            capitalise(&d.state.to_string().replace("_", "-"))
        )
    }));

    format!(
        "<td class=\"{}\" title=\"{}\"><a href=\"#{}\">{}</a></td>",
        p.latest().state,
        html_escape::encode_double_quoted_attribute(&format!("{target}, {}", p.venue())),
        html_escape::encode_double_quoted_attribute(target),
        Vec::from_iter(content.iter().map(html_escape::encode_text)).join("<br>")
    )
}

fn generate_table(metadata: &MetadataMap) -> Vec<String> {
    let mut out = vec![String::from("<table>")];

    let mut venues = BTreeSet::new();
    for m in metadata.data().values() {
        for p in m.publications().publications() {
            venues.insert(p.venue());
        }
    }

    out.extend(generate_header(&venues));

    out.push(String::from("<tbody>"));
    for (target, m) in metadata.data() {
        out.extend(generate_row(target, &venues, m));
    }
    out.extend([String::from("</tbody>"), String::from("</table>")]);

    out
}

fn generate_details(metadata: &MetadataMap) -> Result<Vec<String>, Box<dyn Error>> {
    let mut out = vec![String::from("<h2>Details</h2>")];
    for (target, m) in metadata.data() {
        if !m.publications().is_empty() {
            out.extend([
                format!(
                    "<h3 id=\"{}\">{}</h3>",
                    html_escape::encode_double_quoted_attribute(target),
                    html_escape::encode_text(target)
                ),
                format!(
                    "<code><pre>{}</pre></code>",
                    html_escape::encode_text(&m.to_json()?)
                ),
            ]);
        }
    }
    Ok(out)
}

fn generate_head() -> Vec<String> {
    vec![
        String::from("<head>"),
        String::from("<meta charset=\"utf-8\">"),
        String::from("<title>Publications</title>"),
        String::from("<style>"),
        String::from("table { border-collapse: collapse; }"),
        String::from("th, td { border: 1px solid; padding: 5px; }"),
        String::from("a:link { color: black; }"),
        String::from("a:visited { color: black; }"),
        format!(
            ".{} {{ background-color: #ffff00; }}",
            State::Submitted.to_string()
        ),
        format!(
            ".{} {{ background-color: #ff6d6d; }}",
            State::Rejected.to_string()
        ),
        format!(
            ".{} {{ background-color: #ff972f; }}",
            State::Withdrawn.to_string()
        ),
        format!(
            ".{} {{ background-color: #cccccc; }}",
            State::Abandoned.to_string()
        ),
        format!(
            ".{} {{ background-color: #729fcf; }}",
            State::Accepted.to_string()
        ),
        format!(
            ".{} {{ background-color: #158466; }}",
            State::SelfPublished.to_string()
        ),
        format!(
            ".{} {{ background-color: #81d41a; }}",
            State::Published.to_string()
        ),
        String::from("</style>"),
        String::from("</head>"),
    ]
}

fn generate_body(metadata: &MetadataMap) -> Result<Vec<String>, Box<dyn Error>> {
    let mut out = vec![
        String::from("<body>"),
        String::from("<h1>Publications</h1>"),
    ];
    out.extend(generate_table(metadata));
    out.extend(generate_details(metadata)?);
    out.push(String::from("</body>"));
    Ok(out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let metadata = MetadataMap::build(BTreeMap::from_iter(
        from_json::<MetadataMap>(&read_to_string(args.metadata_file)?)?
            .data()
            .iter()
            .filter(|(_, m)| !m.publications().is_empty())
            .map(|(target, m)| (target.clone(), m.clone())),
    ))?;

    let mut out = vec![
        String::from("<!doctype html>"),
        String::from("<html lang=\"en-GB\">"),
    ];

    out.extend(generate_head());
    out.extend(generate_body(&metadata)?);

    out.push(String::from("</html>"));

    write(args.out_file, out.join("\n") + "\n")?;
    Ok(())
}
