use clap::Parser;
use markdown::args;
use std::error::Error;
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

struct Problem {
    row: usize,
    col: usize,
    problem: String,
}

fn lint(data: &str) -> Vec<Problem> {
    let mut problems = Vec::new();

    for (row, line) in data.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (col, &c) in chars.iter().enumerate() {
            if CURLY_QUOTES.contains(c) && (col == 0 || chars[col - 1] != '\\') {
                problems.push(Problem {
                    row,
                    col,
                    problem: String::from(CURLY_QUOTE_MSG),
                });
            }
        }

        for (bad_char, name, replacement) in BAD_CHARS {
            if let Some(col) = chars.iter().position(|&c| c == bad_char) {
                problems.push(Problem {
                    row,
                    col,
                    problem: format!("Literal {} must be replaced with '{}'", name, replacement),
                });
            }
        }
    }

    problems
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let problems = lint(&read_to_string(args.in_file)?);

    if !problems.is_empty() {
        let mut msg = vec![String::from("ERROR: linting failed")];
        for p in problems {
            msg.push(format!(
                "row {} col {}: {}",
                p.row + 1,
                p.col + 1,
                p.problem
            ));
        }
        eprintln!("{}\n\n", msg.join("\n\n"));
        return Err("linting failed".into());
    }

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
