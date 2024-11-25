use clap::Parser;
use markdown::args::{non_empty, KeyValue};
use markdown::bazel::Label;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fs::{read_to_string, write};

const INCLUDE: &str = "!include";

const INCLUDE_MSG: &str = "Incorrectly-formatted include. Must be '!include \
<md_file label>' where label is in deps, e.g. '!include //foo:bar'.";

const IMAGE_MSG: &str = "Incorrectly-formatted image. Must be \
'![<text>](<label>[ \"text\"])' where label is in 'images', e.g. \
'![foo](//foo:bar)'.";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = non_empty())]
    in_file: String,

    #[arg(value_parser = non_empty())]
    out_file: String,

    current_package: String,

    #[arg(long = "dep")]
    deps: Vec<KeyValue>,

    #[arg(long = "image")]
    images: Vec<KeyValue>,
}

struct Problem {
    row: usize,
    col: usize,
    problem: String,
}

#[derive(Clone, Debug)]
struct LineProblem {
    col: usize,
    problem: String,
}

struct ReplacementResult {
    result: Result<Option<String>, Vec<LineProblem>>,
    deps_used: HashSet<String>,
}

fn process_include(
    line: &str,
    deps: &HashMap<String, String>,
    current_package: &str,
) -> ReplacementResult {
    let Some(raw_label) = line.strip_prefix(INCLUDE) else {
        return ReplacementResult {
            result: Ok(None),
            deps_used: HashSet::new(),
        };
    };

    if !raw_label.starts_with(' ') {
        return ReplacementResult {
            result: Err(vec![LineProblem {
                col: 0,
                problem: format!("Include statement must be followed by a space: {line}"),
            }]),
            deps_used: HashSet::new(),
        };
    }
    let raw_label = raw_label.trim_start_matches(' ');

    match Label::canonicalise(raw_label, current_package) {
        Ok(label) => {
            let label = format!("{}:{}", label.package(), label.target());
            if let Some(replacement) = deps.get(&label) {
                return ReplacementResult {
                    result: Ok(Some(format!("!include {replacement}"))),
                    deps_used: HashSet::from([label]),
                };
            }
            ReplacementResult {
                result: Err(vec![LineProblem {
                    col: 0,
                    problem: format!("{INCLUDE_MSG} {label}"),
                }]),
                deps_used: HashSet::from([label]),
            }
        }
        Err(e) => ReplacementResult {
            result: Err(vec![LineProblem {
                col: 0,
                problem: format!("{INCLUDE_MSG} {e}"),
            }]),
            deps_used: HashSet::new(),
        },
    }
}

fn process_images(
    line: &str,
    images: &HashMap<String, String>,
    current_package: &str,
) -> ReplacementResult {
    let char_indices: Vec<usize> = line.char_indices().map(|(i, _)| i).collect();
    let mut problems = Vec::new();
    let mut labels = HashSet::new();
    let mut replacements = BTreeMap::new();

    let re = Regex::new(r"!\[[^\]]*\]\(([^\)]+)\)").unwrap();
    let captures: Vec<regex::Captures<'_>> = re.captures_iter(line).collect();
    if captures.is_empty() {
        return ReplacementResult {
            result: Ok(None),
            deps_used: labels,
        };
    }

    for capture in captures.iter().map(|c| c.get(1).unwrap()) {
        let text = capture.as_str();
        let col = char_indices
            .iter()
            .position(|&i| i == capture.start())
            .unwrap();

        let (raw_label, title) = {
            if let Some((raw_label, title)) = text.split_once(' ') {
                if title.is_empty() {
                    (raw_label, None)
                } else {
                    (raw_label, Some(title))
                }
            } else {
                (text, None)
            }
        };

        match Label::canonicalise(raw_label, current_package) {
            Ok(label) => {
                let label = format!("{}:{}", label.package(), label.target());
                labels.insert(label.clone());
                if let Some(replacement) = images.get(&label) {
                    let replacement = {
                        if let Some(title) = title {
                            format!("{replacement} {title}")
                        } else {
                            replacement.to_string()
                        }
                    };
                    replacements.insert(text, replacement);
                } else {
                    problems.push(LineProblem {
                        col,
                        problem: format!("{IMAGE_MSG} {label}"),
                    });
                }
            }
            Err(e) => problems.push(LineProblem {
                col,
                problem: format!("{IMAGE_MSG} {e}"),
            }),
        }
    }

    if !problems.is_empty() {
        return ReplacementResult {
            result: Err(problems),
            deps_used: labels,
        };
    }

    let mut line = String::from(line);
    for (text, replacement) in replacements {
        let re = Regex::new(format!(r"!\[([^\]]*)\]\({}\)", regex::escape(text)).as_str()).unwrap();
        line = re
            .replace_all(&line, format!("![${{1}}]({replacement})"))
            .to_string();
    }

    ReplacementResult {
        result: Ok(Some(line)),
        deps_used: labels,
    }
}

fn check_strict_deps(
    used: &BTreeSet<String>,
    declared: &BTreeSet<String>,
    name: &str,
) -> Result<(), String> {
    if used != declared {
        let used_only = used - declared;
        let declared_only = declared - used;
        let mut msg = vec![format!("Used {name} do not match declared {name}")];
        if !used_only.is_empty() {
            msg.push(String::from("Used but not declared"));
            msg.extend(used_only.iter().map(|d| format!("  //{d}")));
        }
        if !declared_only.is_empty() {
            msg.push(String::from("Declared but not used"));
            msg.extend(declared_only.iter().map(|d| format!("  //{d}")));
        }
        return Err(msg.join("\n"));
    }
    Ok(())
}

fn preprocess(
    data: &mut [String],
    deps: &HashMap<String, String>,
    images: &HashMap<String, String>,
    current_package: &str,
) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut used_deps = BTreeSet::new();
    let declared_deps = BTreeSet::from_iter(deps.keys().map(String::from));
    let mut used_images = BTreeSet::new();
    let declared_images = BTreeSet::from_iter(images.keys().map(String::from));

    for (row, line) in data.iter_mut().enumerate() {
        let r = process_include(line, deps, current_package);
        used_deps.extend(r.deps_used);
        match r.result {
            Ok(new_line) => {
                if let Some(new_line) = new_line {
                    // Since an include takes up a whole line, we don't need to
                    // check anything else if we found one.
                    *line = new_line;
                    continue;
                }
            }
            Err(ps) => {
                problems.extend(ps.into_iter().map(|p| Problem {
                    row,
                    col: p.col,
                    problem: p.problem,
                }));
            }
        }

        let r = process_images(line, images, current_package);
        used_images.extend(r.deps_used);
        match r.result {
            Ok(new_line) => {
                if let Some(new_line) = new_line {
                    *line = new_line;
                }
            }
            Err(ps) => {
                problems.extend(ps.into_iter().map(|p| Problem {
                    row,
                    col: p.col,
                    problem: p.problem,
                }));
            }
        }
    }

    if let Err(problem) = check_strict_deps(&used_deps, &declared_deps, "deps") {
        problems.push(Problem {
            row: 0,
            col: 0,
            problem,
        });
    }

    if let Err(problem) = check_strict_deps(&used_images, &declared_images, "images") {
        problems.push(Problem {
            row: 0,
            col: 0,
            problem,
        });
    }

    problems
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut data: Vec<String> = read_to_string(args.in_file)?
        .lines()
        .map(String::from)
        .collect();

    let deps: HashMap<String, String> =
        HashMap::from_iter(args.deps.into_iter().map(|kv| (kv.key, kv.value)));
    let images: HashMap<String, String> =
        HashMap::from_iter(args.images.into_iter().map(|kv| (kv.key, kv.value)));

    let problems = preprocess(&mut data, &deps, &images, &args.current_package);

    if !problems.is_empty() {
        let mut msg = vec![String::from("ERROR: markdown preprocessing failed")];
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

    write(args.out_file, data.join("\n") + "\n")?;
    Ok(())
}

#[cfg(test)]
mod test_preprocess {
    use super::{
        check_strict_deps, preprocess, process_images, process_include, BTreeSet, HashMap, HashSet,
    };

    #[test]
    fn test_process_include() {
        let deps = HashMap::from([
            (String::from("foo:bar"), String::from("foo/bar.json")),
            (String::from("baz:quux"), String::from("baz/quux.json")),
        ]);

        // No include
        let r = process_include("foo bar", &deps, "foo");
        assert!(r.result.unwrap().is_none());
        assert!(r.deps_used.is_empty());

        // Good include
        let r = process_include("!include :bar", &deps, "foo");
        assert_eq!(r.result.unwrap().unwrap(), "!include foo/bar.json");
        assert_eq!(r.deps_used, HashSet::from([String::from("foo:bar")]));

        // Good include with extra spaces
        let r = process_include("!include     :bar", &deps, "foo");
        assert_eq!(r.result.unwrap().unwrap(), "!include foo/bar.json");
        assert_eq!(r.deps_used, HashSet::from([String::from("foo:bar")]));

        // Try to use unknown dependency
        let r = process_include("!include :baz", &deps, "foo");
        assert!(!r.result.unwrap_err().is_empty());
        assert_eq!(r.deps_used, HashSet::from([String::from("foo:baz")]));

        // Invalid include
        let r = process_include("!include", &deps, "foo");
        assert!(!r.result.unwrap_err().is_empty());
        assert!(r.deps_used.is_empty());

        // Invalid label
        let r = process_include("!include a:b:", &deps, "foo");
        assert!(!r.result.unwrap_err().is_empty());
        assert!(r.deps_used.is_empty());
    }

    #[test]
    fn test_process_images() {
        let images = HashMap::from([
            (String::from("foo:bar"), String::from("foo/bar.jpg")),
            (
                String::from("baz/quux:quux"),
                String::from("baz/quux/quux.png"),
            ),
        ]);

        // No images
        let r = process_images("Foo bar baz quux [link](foo)", &images, "foo");
        assert!(r.result.unwrap().is_none());
        assert!(r.deps_used.is_empty());

        // One image
        let r = process_images("Foo ![bar](//foo:bar)", &images, "foo");
        assert_eq!(r.result.unwrap().unwrap(), "Foo ![bar](foo/bar.jpg)");
        assert_eq!(r.deps_used, HashSet::from([String::from("foo:bar")]));

        // One image, title and attributes
        let r = process_images("Foo ![bar](//foo:bar \"baz\\quux\"){.quux}", &images, "foo");
        assert_eq!(
            r.result.unwrap().unwrap(),
            "Foo ![bar](foo/bar.jpg \"baz\\quux\"){.quux}"
        );
        assert_eq!(r.deps_used, HashSet::from([String::from("foo:bar")]));

        // # Multiple images and duplicates
        let r = process_images(
            "Foo ![bar](:bar) bar ![quux](//baz/quux) baz ![bar](:bar) ![bar](:bar \"baz\"){.quux}",
            &images,
            "foo",
        );
        assert_eq!(
            r.result.unwrap().unwrap(),
            "Foo ![bar](foo/bar.jpg) bar ![quux](baz/quux/quux.png) baz ![bar](foo/bar.jpg) \
![bar](foo/bar.jpg \"baz\"){.quux}",
        );
        assert_eq!(
            r.deps_used,
            HashSet::from([String::from("foo:bar"), String::from("baz/quux:quux")])
        );

        // Try to use unknown image
        let r = process_images("Foo ![bar](:bar) bar ![quux](:quux)", &images, "foo");
        assert_eq!(r.result.clone().unwrap_err().len(), 1);
        assert_eq!(r.result.unwrap_err()[0].col, 29);
        assert_eq!(
            r.deps_used,
            HashSet::from([String::from("foo:bar"), String::from("foo:quux")])
        );

        // Invalid label
        let r = process_images("Foo ![bar](:bar:)", &images, "foo");
        assert_eq!(r.result.clone().unwrap_err().len(), 1);
        assert_eq!(r.result.unwrap_err()[0].col, 11);
        assert!(r.deps_used.is_empty());
    }

    #[test]
    fn test_check_strict_deps() {
        // OK
        assert!(check_strict_deps(
            &BTreeSet::from([String::from("a"), String::from("b")]),
            &BTreeSet::from([String::from("a"), String::from("b")]),
            "foo",
        )
        .is_ok());

        // Used but not declared
        assert!(check_strict_deps(
            &BTreeSet::from([String::from("a"), String::from("b")]),
            &BTreeSet::from([String::from("a")]),
            "foo",
        )
        .is_err());

        // Declared but not used
        assert!(check_strict_deps(
            &BTreeSet::from([String::from("a")]),
            &BTreeSet::from([String::from("a"), String::from("b")]),
            "foo",
        )
        .is_err());

        // Both
        assert!(check_strict_deps(
            &BTreeSet::from([String::from("a"), String::from("c")]),
            &BTreeSet::from([String::from("a"), String::from("d")]),
            "foo",
        )
        .is_err());
    }

    #[test]
    fn test_preprocess() {
        let deps = HashMap::from([
            (String::from("foo:bar"), String::from("foo/bar.json")),
            (String::from("baz:quux"), String::from("baz/quux.json")),
        ]);
        let images = HashMap::from([(String::from("a:yay"), String::from("a/yay.jpg"))]);

        const GOOD: &str = "Foo bar.

!include {}

!include {}

An image ![foo]({} \"bar\"){.baz} goes here.
";

        fn make_data(include1: &str, include2: &str, image: &str) -> String {
            GOOD.replacen("{}", include1, 1)
                .replacen("{}", include2, 1)
                .replacen("{}", image, 1)
        }

        // OK
        let mut data: Vec<String> = Vec::from_iter(
            make_data("//foo:bar", "//baz:quux", ":yay")
                .split("\n")
                .map(String::from),
        );
        let problems = preprocess(&mut data, &deps, &images, "a");
        assert!(problems.is_empty());
        assert_eq!(
            data.join("\n"),
            make_data("foo/bar.json", "baz/quux.json", "a/yay.jpg")
        );

        // Bad include and bad image
        let mut data: Vec<String> = Vec::from_iter(
            make_data("//foo:bar", "//blah:yay", "//baz:quux")
                .split("\n")
                .map(String::from),
        );
        let problems = preprocess(&mut data, &deps, &images, "a");
        assert_eq!(problems.len(), 4);
        assert_eq!(
            data.join("\n"),
            make_data("foo/bar.json", "//blah:yay", "//baz:quux")
        );
    }
}
