use clap::Parser;
use markdown::args;
use markdown::problems::{Problems, RowColProblem};
use std::error::Error;
use std::fmt::Display;
use std::fs::{read_to_string, write};

const CURLY_QUOTES: &str = "“”‘’";
const CURLY_QUOTE_MSG: &str = "Literal curly quotes must be backslash-escaped";
const BAD_CHARS: [(char, &str, &str); 3] = [
    ('–', "en-dashes", "--"),
    ('—', "em-dashes", "---"),
    ('…', "ellipses", "..."),
];

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = args::non_empty())]
    in_file: String,

    #[arg(value_parser = args::non_empty())]
    out_file: String,
}

fn lint(data: &str) -> Vec<RowColProblem> {
    let mut problems = Vec::new();

    for (row, line) in data.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (col, &c) in chars.iter().enumerate() {
            if CURLY_QUOTES.contains(c) && (col == 0 || chars[col - 1] != '\\') {
                problems.push(RowColProblem::new(row, col, CURLY_QUOTE_MSG));
            }
        }

        for (bad_char, name, replacement) in BAD_CHARS {
            if let Some(col) = chars.iter().position(|&c| c == bad_char) {
                problems.push(RowColProblem::new(
                    row,
                    col,
                    &format!("Literal {} must be replaced with '{}'", name, replacement),
                ));
            }
        }
    }

    problems
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut problems = Problems::new("linting failed");
    problems.extend(
        lint(&read_to_string(args.in_file)?)
            .into_iter()
            .map(|p| -> Box<dyn Display> { Box::new(p) }),
    );
    problems.check();

    write(args.out_file, "OK\n")?;
    Ok(())
}

#[cfg(test)]
mod custom_lint_test {
    use crate::lint;

    #[test]
    fn test_lint() {
        // OK
        assert!(lint(
            "Foo bar.

\\“Lots \\”of \\‘quotes\\’.

Some -- dashes---
"
        )
        .is_empty());

        assert!(!lint("“").is_empty());
        assert!(!lint("”").is_empty());
        assert!(!lint("‘").is_empty());
        assert!(!lint("’").is_empty());
        assert!(!lint("–").is_empty());
        assert!(!lint("—").is_empty());
        assert!(!lint("…").is_empty());
    }
}
