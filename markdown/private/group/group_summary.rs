#![allow(warnings)]

use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Arg, ArgAction, ArgGroup, Command, Id,
};
use markdown::args;
use markdown::json::from_json;
use markdown::metadata::MetadataMap;
use regex::Regex;
use serde::Serialize;
use std::error::Error;
use std::fs::read_to_string;
use std::io::stdout;
use tabled::{
    settings::{object::Columns, Alignment, Settings, Style},
    Table, Tabled,
};

const TARGET: &str = "target";
const TITLE: &str = "title";
const AUTHOR: &str = "author";
const RAW_DATE: &str = "raw date";
const DATE: &str = "date";
const WORDCOUNT: &str = "wordcount";
const POETRY_LINES: &str = "poetry lines";
const FINISHED: &str = "finished";
const PUBLICATION: &str = "publication";
const VERSION: &str = "version";
const STATUS: &str = "status";

#[derive(Clone, Copy)]
enum Getter {
    String(fn(&Row) -> &str),
    Int(fn(&Row) -> u32),
}

impl Getter {
    pub fn to_string(&self, row: &Row) -> String {
        match self {
            Getter::String(f) => f(row).to_string(),
            Getter::Int(f) => f(row).to_string(),
        }
    }
}

const COLUMNS: [(&str, Getter); 11] = [
    (TARGET, Getter::String(Row::target)),
    (TITLE, Getter::String(Row::title)),
    (AUTHOR, Getter::String(Row::author)),
    (RAW_DATE, Getter::String(Row::raw_date)),
    (DATE, Getter::String(Row::date)),
    (WORDCOUNT, Getter::Int(Row::wordcount)),
    (POETRY_LINES, Getter::Int(Row::poetry_lines)),
    (FINISHED, Getter::String(Row::finished)),
    (PUBLICATION, Getter::String(Row::publication)),
    (VERSION, Getter::String(Row::version)),
    (STATUS, Getter::String(Row::status)),
];

#[derive(Serialize, Tabled)]
struct Row {
    target: String,
    title: String,
    author: String,
    #[tabled(rename = "raw date")]
    raw_date: String,
    date: String,
    wordcount: u32,
    #[tabled(rename = "poetry lines")]
    poetry_lines: u32,
    finished: String,
    publication: String,
    version: String,
    status: String,
}

impl Row {
    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn raw_date(&self) -> &str {
        &self.raw_date
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn wordcount(&self) -> u32 {
        self.wordcount
    }

    pub fn poetry_lines(&self) -> u32 {
        self.poetry_lines
    }

    pub fn finished(&self) -> &str {
        &self.finished
    }

    pub fn publication(&self) -> &str {
        &self.publication
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

#[derive(Clone)]
struct Filter {
    getter: Getter,
    regex: Regex,
}

impl Filter {
    pub fn build(getter: Getter, regex: &str) -> Result<Filter, String> {
        if regex.is_empty() {
            return Err("regex must be non-empty".into());
        }
        let re = match Regex::new(regex) {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()),
        };
        Ok(Filter { getter, regex: re })
    }

    pub fn matches(&self, r: &Row) -> bool {
        self.regex.is_match(&self.getter.to_string(r))
    }
}

#[derive(Clone)]
struct PartialFilter {
    getter: Getter,
}

impl PartialFilter {
    pub fn new(getter: Getter) -> PartialFilter {
        PartialFilter { getter }
    }
}

impl clap::builder::TypedValueParser for PartialFilter {
    type Value = Filter;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let Some(val) = value.to_str() else {
            return Err(clap::Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd));
        };
        match Filter::build(self.getter, val) {
            Ok(f) => Ok(f),
            Err(e) => {
                let mut out = clap::Error::new(ErrorKind::InvalidValue);
                out.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(e.to_string()),
                );
                Err(out)
            }
        }
    }
}

#[derive(Clone)]
struct Sorter {
    field: String,
    getter: Getter,
    reverse: bool,
}

impl Sorter {
    pub fn new(field: &str, getter: Getter, reverse: bool) -> Sorter {
        Sorter {
            field: field.to_string(),
            getter,
            reverse,
        }
    }

    fn string_key(&self, f: fn(&Row) -> &str, row: &Row) -> String {
        if self.field == DATE {
            let mut dates = Vec::from_iter(f(row).split(',').map(|s| s.trim()));
            dates.sort();
            if self.reverse {
                dates.reverse();
            }
            return dates.join(", ");
        }
        f(row).to_ascii_lowercase()
    }

    pub fn sort(&self, data: &mut Vec<Row>) {
        match self.getter {
            Getter::String(f) => {
                data.sort_by_key(|r| self.string_key(f, r));
            }
            Getter::Int(f) => {
                data.sort_by_key(f);
            }
        };
        if self.reverse {
            data.reverse();
        }
    }
}

struct Args {
    metadata_file: String,
    raw: bool,
    includes: Vec<Filter>,
    excludes: Vec<Filter>,
    sorter: Sorter,
}

fn parse_args() -> Args {
    let mut command = Command::new("group_summary")
        .about(
            "Summarise the contents of the group. To be displayed, a row \
must match any include and no excludes; by default all rows are displayed",
        )
        .arg(
            Arg::new("metadata_file")
                .required(true)
                .value_parser(args::non_empty()),
        )
        .arg(
            Arg::new("raw")
                .long("raw")
                .action(ArgAction::SetTrue)
                .help("output CSV instead of a human-readable table"),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .action(ArgAction::SetTrue)
                .help("reverse sorting direction"),
        )
        .group(ArgGroup::new("includes").required(false).multiple(true))
        .group(ArgGroup::new("excludes").required(false).multiple(true))
        .group(ArgGroup::new("sorters").required(false).multiple(false));

    for (field, getter) in COLUMNS {
        let name = field.replace(" ", "-");
        command = command
            .arg(
                Arg::new(format!("include-{name}"))
                    .long(format!("include-{name}"))
                    .group("includes")
                    .value_parser(PartialFilter::new(getter))
                    .help(format!("include rows matching regex on {field}")),
            )
            .arg(
                Arg::new(format!("exclude-{name}"))
                    .long(format!("exclude-{name}"))
                    .group("excludes")
                    .value_parser(PartialFilter::new(getter))
                    .help(format!("exclude rows matching regex on {field}")),
            )
            .arg(
                Arg::new(format!("sort-{name}"))
                    .long(format!("sort-{name}"))
                    .group("sorters")
                    .action(ArgAction::SetTrue)
                    .help(format!("sort by {field}")),
            );
    }

    let matches = command.get_matches();
    let sort_column = matches
        .get_many::<Id>("sorters")
        .into_iter()
        .flatten()
        .map(|id| {
            if matches.get_flag(id.as_str()) {
                Some(id.as_str().strip_prefix("sort-").unwrap())
            } else {
                None
            }
        })
        .flatten()
        .next()
        .unwrap_or(TARGET)
        .replace("-", " ");

    Args {
        metadata_file: matches
            .get_one::<String>("metadata_file")
            .unwrap()
            .to_string(),
        raw: matches.get_flag("raw"),
        includes: matches
            .get_many::<Id>("includes")
            .into_iter()
            .flatten()
            .map(|id| matches.get_one::<Filter>(id.as_str()))
            .flatten()
            .map(|f| f.clone())
            .collect(),
        excludes: matches
            .get_many::<Id>("excludes")
            .into_iter()
            .flatten()
            .map(|id| matches.get_one::<Filter>(id.as_str()))
            .flatten()
            .map(|f| f.clone())
            .collect(),
        sorter: Sorter::new(
            &sort_column,
            COLUMNS
                .iter()
                .find(|(name, _)| *name == sort_column)
                .unwrap()
                .1,
            matches.get_flag("reverse"),
        ),
    }
}

fn should_include(row: &Row, includes: &Vec<Filter>, excludes: &Vec<Filter>) -> bool {
    if excludes.iter().any(|f| f.matches(row)) {
        return false;
    }
    if includes.is_empty() {
        return true;
    }
    includes.iter().any(|f| f.matches(row))
}

fn sanitize(s: &str) -> String {
    s.replace("\n", "\\n")
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();

    let metadata = from_json::<MetadataMap>(&read_to_string(args.metadata_file)?)?;
    let mut data = Vec::new();

    for (target, m) in metadata.data() {
        let publication = if !m.publications().is_empty() {
            if let Some(state) = m.publications().highest_active_state() {
                state.to_string().replace("_", "-")
            } else {
                String::from("attempted")
            }
        } else {
            String::new()
        };

        let row = Row {
            target: target.clone(),
            title: sanitize(m.title().unwrap_or(&String::new())),
            author: sanitize(&m.authors().join(", ")),
            raw_date: sanitize(m.date().unwrap_or(&String::new())),
            date: sanitize(&m.parsed_dates().dates().join(", ")),
            wordcount: m.wordcount(),
            poetry_lines: m.poetry_lines(),
            finished: if m.finished() {
                String::from("yes")
            } else {
                String::from("no")
            },
            publication,
            version: m.version().to_string(),
            status: if m.version().contains("dirty") {
                String::from("DIRTY")
            } else {
                String::from("ok")
            },
        };

        if should_include(&row, &args.includes, &args.excludes) {
            data.push(row);
        }
    }

    args.sorter.sort(&mut data);

    if args.raw {
        let mut out = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(stdout());
        out.write_record(COLUMNS.map(|(name, _)| name))?;
        for row in data {
            out.serialize(row)?;
        }
    } else {
        println!(
            "{}",
            Table::new(data)
                .with(Style::markdown())
                .modify(Columns::single(5), Alignment::right())
                .modify(Columns::single(6), Alignment::right())
        )
    }
    Ok(())
}
