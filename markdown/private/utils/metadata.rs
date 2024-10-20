use crate::{json, json::Json, publications::Publications};
use chrono::naive::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashSet;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Version {
    pub version: String,
    pub repo: String,
}

impl Version {
    pub fn new(version: &str, repo: &str) -> Version {
        Version {
            version: String::from(version),
            repo: String::from(repo),
        }
    }
}

impl Json for Version {}

#[derive(Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct SourceHash {
    pub source_hash: String,
}

impl SourceHash {
    pub fn new(source_hash: &str) -> SourceHash {
        SourceHash {
            source_hash: String::from(source_hash),
        }
    }
}

impl Json for SourceHash {}

#[derive(Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
#[serde(rename_all = "kebab-case")]
pub struct ParsedDateSet {
    #[validate(custom(function = "ParsedDateSet::validate_sorted_date_set"))]
    parsed_dates: Vec<String>,
}

impl ParsedDateSet {
    pub fn build(parsed_dates: Vec<String>) -> Result<ParsedDateSet, ValidationErrors> {
        let pd = ParsedDateSet { parsed_dates };
        pd.validate()?;
        Ok(pd)
    }

    pub fn dates(&self) -> &Vec<String> {
        &self.parsed_dates
    }

    fn valid_date(d: &str) -> bool {
        let re =
            Regex::new(r"^(?<year>[0-9]{4})(/(?<month>[0-9]{2})(/(?<day>[0-9]{2}))?)?$").unwrap();
        let Some(caps) = re.captures(d) else {
            return false;
        };

        let year = match caps.name("year") {
            Some(year) => year.as_str(),
            None => {
                return false;
            }
        };
        let month = caps.name("month").map_or("01", |m| m.as_str());
        let day = caps.name("day").map_or("01", |m| m.as_str());

        let d = format!("{year}/{month}/{day}");
        NaiveDate::parse_from_str(&d, "%Y/%m/%d").is_ok()
    }

    fn validate_sorted_date_set(parsed_dates: &Vec<String>) -> Result<(), ValidationError> {
        if !parsed_dates.iter().all(|s| Self::valid_date(s)) {
            return Err(ValidationError::new(
                "all entries must be in YYYY, YYYY/MM or YYYY/MM/DD format",
            ));
        }

        let set: HashSet<&String> = parsed_dates.iter().collect();
        let mut values: Vec<String> = set.into_iter().cloned().collect();
        values.sort();

        if *parsed_dates != values {
            return Err(ValidationError::new("elements must be unique and sorted"));
        }

        Ok(())
    }
}

impl Json for ParsedDateSet {}

#[derive(Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct ParsedDates {
    #[validate(nested)]
    parsed_dates: ParsedDateSet,
}

impl ParsedDates {
    pub fn build(parsed_dates: ParsedDateSet) -> Result<ParsedDates, ValidationErrors> {
        let pd = ParsedDates { parsed_dates };
        pd.validate()?;
        Ok(pd)
    }

    pub fn dates(&self) -> &Vec<String> {
        &self.parsed_dates.parsed_dates
    }
}

impl Json for ParsedDates {}

#[derive(Serialize, Deserialize, Validate, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct Identifier {
    pub scheme: String,
    pub text: String,
}

impl Identifier {
    pub fn new(scheme: &str, text: &str) -> Identifier {
        Identifier {
            scheme: String::from(scheme),
            text: String::from(text),
        }
    }
}

impl Json for Identifier {}

#[derive(Serialize, Deserialize, Default, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct InputMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "json::deserialize::str_or_seq")]
    author: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "json::is_false")]
    finished: bool,

    #[serde(default)]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Publications::is_empty")]
    publications: Publications,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(nested)]
    identifier: Vec<Identifier>,
}

impl InputMetadata {
    pub fn build(
        title: Option<&str>,
        authors: Vec<String>,
        date: Option<&str>,
        notes: Option<&str>,
        finished: bool,
        publications: Publications,
        identifiers: Vec<Identifier>,
    ) -> Result<InputMetadata, ValidationErrors> {
        let m = InputMetadata {
            title: title.map(str::to_string),
            author: authors,
            date: date.map(str::to_string),
            notes: notes.map(str::to_string),
            finished,
            publications,
            identifier: identifiers,
        };
        m.validate()?;
        Ok(m)
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn authors(&self) -> &Vec<String> {
        &self.author
    }

    pub fn date(&self) -> Option<&String> {
        self.date.as_ref()
    }

    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn publications(&self) -> &Publications {
        &self.publications
    }

    pub fn identifiers(&self) -> &Vec<Identifier> {
        &self.identifier
    }
}

impl Json for InputMetadata {}

#[derive(Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct OutputMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "json::deserialize::str_or_seq")]
    author: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "json::is_false")]
    finished: bool,

    #[serde(default)]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Publications::is_empty")]
    publications: Publications,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(nested)]
    identifier: Vec<Identifier>,

    #[serde(deserialize_with = "json::deserialize::int_or_str")]
    wordcount: u32,

    #[serde(deserialize_with = "json::deserialize::int_or_str")]
    poetry_lines: u32,

    lang: String,
    version: String,
    repo: String,
    source_hash: String,

    #[validate(nested)]
    parsed_dates: ParsedDateSet,
}

impl OutputMetadata {
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        title: Option<&str>,
        authors: Vec<String>,
        date: Option<&str>,
        notes: Option<&str>,
        finished: bool,
        publications: Publications,
        identifiers: Vec<Identifier>,
        wordcount: u32,
        poetry_lines: u32,
        lang: &str,
        version: &str,
        repo: &str,
        source_hash: &str,
        parsed_dates: ParsedDateSet,
    ) -> Result<OutputMetadata, ValidationErrors> {
        let m = OutputMetadata {
            title: title.map(str::to_string),
            author: authors,
            date: date.map(str::to_string),
            notes: notes.map(str::to_string),
            finished,
            publications,
            identifier: identifiers,
            wordcount,
            poetry_lines,
            lang: String::from(lang),
            version: String::from(version),
            repo: String::from(repo),
            source_hash: String::from(source_hash),
            parsed_dates,
        };
        m.validate()?;
        Ok(m)
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn authors(&self) -> &Vec<String> {
        &self.author
    }

    pub fn date(&self) -> Option<&String> {
        self.date.as_ref()
    }

    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn publications(&self) -> &Publications {
        &self.publications
    }

    pub fn identifiers(&self) -> &Vec<Identifier> {
        &self.identifier
    }

    pub fn wordcount(&self) -> u32 {
        self.wordcount
    }

    pub fn poetry_lines(&self) -> u32 {
        self.poetry_lines
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    pub fn source_hash(&self) -> &str {
        &self.source_hash
    }

    pub fn parsed_dates(&self) -> &ParsedDateSet {
        &self.parsed_dates
    }
}

impl Json for OutputMetadata {}

#[derive(Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct MetadataMap {
    #[validate(nested)]
    data: BTreeMap<String, OutputMetadata>,
}

impl MetadataMap {
    pub fn build(data: BTreeMap<String, OutputMetadata>) -> Result<MetadataMap, ValidationErrors> {
        let m = MetadataMap { data };
        m.validate()?;
        Ok(m)
    }

    pub fn data(&self) -> &BTreeMap<String, OutputMetadata> {
        &self.data
    }
}

impl Json for MetadataMap {}

#[cfg(test)]
mod test_utils {
    use chrono::NaiveDate;

    pub fn ymd(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }
}

#[cfg(test)]
mod version_test {
    use super::Version;
    use crate::json::{from_str, Json};

    #[test]
    fn test_serialization() {
        assert_eq!(
            Version::new("foo", "bar").to_json().unwrap(),
            r#"{
    "repo": "bar",
    "version": "foo"
}"#
        )
    }

    #[test]
    fn test_deserialization() {
        let v: Version = from_str(
            r#"{
    "repo": "bar",
    "version": "foo"
}"#,
        )
        .unwrap();
        assert_eq!(v.version, "foo");
        assert_eq!(v.repo, "bar");
    }
}

#[cfg(test)]
mod source_hash_test {
    use super::SourceHash;
    use crate::json::{from_str, Json};

    #[test]
    fn test_serialization() {
        assert_eq!(
            SourceHash::new("foo").to_json().unwrap(),
            r#"{
    "source-hash": "foo"
}"#
        )
    }

    #[test]
    fn test_deserialization() {
        let h: SourceHash = from_str(
            r#"{
    "source-hash": "foo"
}"#,
        )
        .unwrap();
        assert_eq!(h.source_hash, "foo");
    }
}

#[cfg(test)]
mod parsed_date_set_test {
    use super::ParsedDateSet;

    #[test]
    fn test_bad() {
        let pd: Result<ParsedDateSet, _> = ParsedDateSet::build(vec![String::from("foo")]);
        assert!(pd.is_err());
    }
}

#[cfg(test)]
mod parsed_dates_test {
    use super::{ParsedDateSet, ParsedDates};
    use crate::json::{from_str, Json};

    #[test]
    fn test_serialization() {
        assert_eq!(
            ParsedDates::build(
                ParsedDateSet::build(vec![String::from("2020"), String::from("2020/01")]).unwrap()
            )
            .unwrap()
            .to_json()
            .unwrap(),
            r#"{
    "parsed-dates": [
        "2020",
        "2020/01"
    ]
}"#
        )
    }

    #[test]
    fn test_deserialization_good() {
        let pd: ParsedDates = from_str(
            r#"{
    "parsed-dates": [
        "2020/01/01",
        "2021",
        "2021/03",
        "2024/06/23"
    ]
}"#,
        )
        .unwrap();
        assert_eq!(pd.dates(), &["2020/01/01", "2021", "2021/03", "2024/06/23"]);
    }

    #[test]
    fn test_deserialization_bad_invalid() {
        let pd: Result<ParsedDates, _> = from_str(
            r#"{
    "parsed-dates": [
        "2020/01/01 10:30:00"
    ]
}"#,
        );
        assert!(pd.is_err());
    }

    #[test]
    fn test_deserialization_bad_duplicates() {
        let pd: Result<ParsedDates, _> = from_str(
            r#"{
    "parsed-dates": [
        "2020/01/01",
        "2020/01/01"
    ]
}"#,
        );
        assert!(pd.is_err());
    }

    #[test]
    fn test_deserialization_bad_unordered() {
        let pd: Result<ParsedDates, _> = from_str(
            r#"{
    "parsed-dates": [
        "2024/06/23",
        "2020/01/01"
    ]
}"#,
        );
        assert!(pd.is_err());
    }
}

#[cfg(test)]
mod identifier_test {
    use super::Identifier;
    use crate::json::{from_str, Json};

    #[test]
    fn test_serialization() {
        assert_eq!(
            Identifier::new("foo", "bar").to_json().unwrap(),
            r#"{
    "scheme": "foo",
    "text": "bar"
}"#
        )
    }

    #[test]
    fn test_deserialization() {
        let i: Identifier = from_str(
            r#"{
    "scheme": "foo",
    "text": "bar"
}"#,
        )
        .unwrap();
        assert_eq!(i.scheme, "foo");
        assert_eq!(i.text, "bar");
    }
}

#[cfg(test)]
mod input_metadata_test {
    use super::{test_utils::ymd, Identifier, InputMetadata};
    use crate::json::{from_str, Json};
    use crate::publications::{Publication, Publications, State};

    #[test]
    fn test_serialization_minimal() {
        assert_eq!(InputMetadata::default().to_json().unwrap(), r#"{}"#)
    }

    #[test]
    fn test_serialization_full() {
        assert_eq!(
            InputMetadata::build(
                Some("foo"),
                vec![String::from("bar"), String::from("baz")],
                Some("quux"),
                Some("blah"),
                true,
                Publications::build(vec![
                    Publication::build(
                        "Book",
                        vec![String::from("foo"), String::from("bar")],
                        Some("baz"),
                        Some("quux"),
                        ymd(2023, 5, 16),
                        ymd(2023, 5, 17),
                        None,
                        None,
                        None,
                        None,
                        ymd(2023, 5, 18),
                    )
                    .unwrap(),
                    Publication::build(
                        "Book2",
                        vec![String::from("foo2"), String::from("bar2")],
                        Some("baz2"),
                        Some("quux2"),
                        ymd(2023, 5, 19),
                        ymd(2023, 5, 20),
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                ])
                .unwrap(),
                vec![Identifier::new("a", "b"), Identifier::new("c", "d")],
            )
            .unwrap()
            .to_json()
            .unwrap(),
            r#"{
    "author": [
        "bar",
        "baz"
    ],
    "date": "quux",
    "finished": true,
    "identifier": [
        {
            "scheme": "a",
            "text": "b"
        },
        {
            "scheme": "c",
            "text": "d"
        }
    ],
    "notes": "blah",
    "publications": [
        {
            "accepted": "2023-05-17",
            "notes": "baz",
            "paid": "quux",
            "published": "2023-05-18",
            "submitted": "2023-05-16",
            "urls": [
                "foo",
                "bar"
            ],
            "venue": "Book"
        },
        {
            "accepted": "2023-05-20",
            "notes": "baz2",
            "paid": "quux2",
            "submitted": "2023-05-19",
            "urls": [
                "foo2",
                "bar2"
            ],
            "venue": "Book2"
        }
    ],
    "title": "foo"
}"#
        )
    }

    #[test]
    fn test_deserialization_minimal() {
        let m: InputMetadata = from_str(r#"{}"#).unwrap();
        assert!(m.title().is_none());
        assert!(m.authors().is_empty());
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
    }

    #[test]
    fn test_deserialization_single_author() {
        let m: InputMetadata = from_str(r#"{"author": "foo"}"#).unwrap();
        assert!(m.title().is_none());
        assert_eq!(m.authors(), &["foo"]);
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
    }

    #[test]
    fn test_deserialization_full() {
        let m: InputMetadata = from_str(
            r#"{
            "author": [
                "bar",
                "baz"
            ],
            "date": "quux",
            "finished": true,
            "identifier": [
                {
                    "scheme": "a",
                    "text": "b"
                },
                {
                    "scheme": "c",
                    "text": "d"
                }
            ],
            "notes": "blah",
            "publications": [
                {
                    "accepted": "2023-05-17",
                    "notes": "baz",
                    "paid": "quux",
                    "published": "2023-05-18",
                    "submitted": "2023-05-16",
                    "urls": [
                        "foo",
                        "bar"
                    ],
                    "venue": "Book"
                },
                {
                    "accepted": "2023-05-20",
                    "notes": "baz2",
                    "paid": "quux2",
                    "submitted": "2023-05-19",
                    "urls": [
                        "foo2",
                        "bar2"
                    ],
                    "venue": "Book2"
                }
            ],
            "title": "foo"
        }"#,
        )
        .unwrap();

        assert_eq!(m.title().unwrap(), "foo");
        assert_eq!(m.authors(), &["bar", "baz"]);
        assert_eq!(m.date().unwrap(), "quux");
        assert_eq!(m.notes().unwrap(), "blah");
        assert!(m.finished());

        let ps = m.publications();
        assert_eq!(ps.publications().len(), 2);
        assert!(ps.active());
        assert_eq!(ps.highest_active_state(), Some(State::Published));

        let p = &ps.publications()[0];
        assert_eq!(p.venue(), "Book");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied(), ymd(2023, 5, 18));
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");

        let p = &ps.publications()[1];
        assert_eq!(p.venue(), "Book2");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 19));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 20));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.urls(), &["foo2", "bar2"]);
        assert_eq!(p.notes().unwrap(), "baz2");
        assert_eq!(p.paid().unwrap(), "quux2");

        assert_eq!(
            m.identifiers(),
            &[Identifier::new("a", "b"), Identifier::new("c", "d")]
        );
    }
}

#[cfg(test)]
mod output_metadata_test {
    use super::{test_utils::ymd, Identifier, OutputMetadata, ParsedDateSet};
    use crate::json::{from_str, Json};
    use crate::publications::{Publication, Publications, State};

    #[test]
    fn test_serialization_full() {
        assert_eq!(
            OutputMetadata::build(
                Some("foo"),
                vec![String::from("bar"), String::from("baz")],
                Some("quux"),
                Some("blah"),
                true,
                Publications::build(vec![
                    Publication::build(
                        "Book",
                        vec![String::from("foo"), String::from("bar")],
                        Some("baz"),
                        Some("quux"),
                        ymd(2023, 5, 16),
                        ymd(2023, 5, 17),
                        None,
                        None,
                        None,
                        None,
                        ymd(2023, 5, 18),
                    )
                    .unwrap(),
                    Publication::build(
                        "Book2",
                        vec![String::from("foo2"), String::from("bar2")],
                        Some("baz2"),
                        Some("quux2"),
                        ymd(2023, 5, 19),
                        ymd(2023, 5, 20),
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                ])
                .unwrap(),
                vec![Identifier::new("a", "b"), Identifier::new("c", "d")],
                10,
                5,
                "blah1",
                "blah2",
                "blah3",
                "blah4",
                ParsedDateSet::build(vec![String::from("2020"), String::from("2020/01")]).unwrap(),
            )
            .unwrap()
            .to_json()
            .unwrap(),
            r#"{
    "author": [
        "bar",
        "baz"
    ],
    "date": "quux",
    "finished": true,
    "identifier": [
        {
            "scheme": "a",
            "text": "b"
        },
        {
            "scheme": "c",
            "text": "d"
        }
    ],
    "lang": "blah1",
    "notes": "blah",
    "parsed-dates": [
        "2020",
        "2020/01"
    ],
    "poetry-lines": 5,
    "publications": [
        {
            "accepted": "2023-05-17",
            "notes": "baz",
            "paid": "quux",
            "published": "2023-05-18",
            "submitted": "2023-05-16",
            "urls": [
                "foo",
                "bar"
            ],
            "venue": "Book"
        },
        {
            "accepted": "2023-05-20",
            "notes": "baz2",
            "paid": "quux2",
            "submitted": "2023-05-19",
            "urls": [
                "foo2",
                "bar2"
            ],
            "venue": "Book2"
        }
    ],
    "repo": "blah3",
    "source-hash": "blah4",
    "title": "foo",
    "version": "blah2",
    "wordcount": 10
}"#
        )
    }

    #[test]
    fn test_deserialization_single_author() {
        let m: OutputMetadata = from_str(
            r#"{
    "author": "foo",
    "lang": "blah1",
    "parsed-dates": [],
    "poetry-lines": 5,
    "repo": "blah3",
    "source-hash": "blah4",
    "version": "blah2",
    "wordcount": 10
}"#,
        )
        .unwrap();
        assert!(m.title().is_none());
        assert_eq!(m.authors(), &["foo"]);
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
        assert_eq!(m.wordcount(), 10);
        assert_eq!(m.poetry_lines(), 5);
        assert_eq!(m.lang(), "blah1");
        assert_eq!(m.version(), "blah2");
        assert_eq!(m.repo(), "blah3");
        assert_eq!(m.source_hash(), "blah4");
        assert!(m.parsed_dates().dates().is_empty());
    }

    #[test]
    fn test_deserialization_str_numbers() {
        let m: OutputMetadata = from_str(
            r#"{
    "author": "foo",
    "lang": "blah1",
    "parsed-dates": [],
    "poetry-lines": "5",
    "repo": "blah3",
    "source-hash": "blah4",
    "version": "blah2",
    "wordcount": "10"
}"#,
        )
        .unwrap();
        assert!(m.title().is_none());
        assert_eq!(m.authors(), &["foo"]);
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
        assert_eq!(m.wordcount(), 10);
        assert_eq!(m.poetry_lines(), 5);
        assert_eq!(m.lang(), "blah1");
        assert_eq!(m.version(), "blah2");
        assert_eq!(m.repo(), "blah3");
        assert_eq!(m.source_hash(), "blah4");
        assert!(m.parsed_dates().dates().is_empty());
    }

    #[test]
    fn test_deserialization_full() {
        let m: OutputMetadata = from_str(
            r#"{
                "author": [
                    "bar",
                    "baz"
                ],
                "date": "quux",
                "finished": true,
                "identifier": [
                    {
                        "scheme": "a",
                        "text": "b"
                    },
                    {
                        "scheme": "c",
                        "text": "d"
                    }
                ],
                "lang": "blah1",
                "notes": "blah",
                "parsed-dates": [
                    "2020",
                    "2020/01"
                ],
                "poetry-lines": 5,
                "publications": [
                    {
                        "accepted": "2023-05-17",
                        "notes": "baz",
                        "paid": "quux",
                        "published": "2023-05-18",
                        "submitted": "2023-05-16",
                        "urls": [
                            "foo",
                            "bar"
                        ],
                        "venue": "Book"
                    },
                    {
                        "accepted": "2023-05-20",
                        "notes": "baz2",
                        "paid": "quux2",
                        "submitted": "2023-05-19",
                        "urls": [
                            "foo2",
                            "bar2"
                        ],
                        "venue": "Book2"
                    }
                ],
                "repo": "blah3",
                "source-hash": "blah4",
                "title": "foo",
                "version": "blah2",
                "wordcount": 10
            }"#,
        )
        .unwrap();

        assert_eq!(m.title().unwrap(), "foo");
        assert_eq!(m.authors(), &["bar", "baz"]);
        assert_eq!(m.date().unwrap(), "quux");
        assert_eq!(m.notes().unwrap(), "blah");
        assert!(m.finished());

        let ps = m.publications();
        assert_eq!(ps.publications().len(), 2);
        assert!(ps.active());
        assert_eq!(ps.highest_active_state(), Some(State::Published));

        let p = &ps.publications()[0];
        assert_eq!(p.venue(), "Book");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied(), ymd(2023, 5, 18));
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");

        let p = &ps.publications()[1];
        assert_eq!(p.venue(), "Book2");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 19));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 20));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.urls(), &["foo2", "bar2"]);
        assert_eq!(p.notes().unwrap(), "baz2");
        assert_eq!(p.paid().unwrap(), "quux2");

        assert_eq!(
            m.identifiers(),
            &[Identifier::new("a", "b"), Identifier::new("c", "d")]
        );

        assert_eq!(m.wordcount(), 10);
        assert_eq!(m.poetry_lines(), 5);
        assert_eq!(m.lang(), "blah1");
        assert_eq!(m.version(), "blah2");
        assert_eq!(m.repo(), "blah3");
        assert_eq!(m.source_hash(), "blah4");

        assert_eq!(m.parsed_dates().dates(), &["2020", "2020/01"]);
    }
}

#[cfg(test)]
mod metadata_map_test {
    use std::collections::BTreeMap;

    use super::{MetadataMap, OutputMetadata, ParsedDateSet};
    use crate::json::{from_str, Json};
    use crate::publications::Publications;

    #[test]
    fn test_serialization() {
        let mut data = BTreeMap::new();
        data.insert(
            String::from("foo"),
            OutputMetadata::build(
                None,
                Vec::new(),
                None,
                None,
                false,
                Publications::build(Vec::new()).unwrap(),
                Vec::new(),
                10,
                5,
                "blah1",
                "blah2",
                "blah3",
                "blah4",
                ParsedDateSet::build(Vec::new()).unwrap(),
            )
            .unwrap(),
        );
        data.insert(
            String::from("bar"),
            OutputMetadata::build(
                None,
                Vec::new(),
                None,
                None,
                false,
                Publications::build(Vec::new()).unwrap(),
                Vec::new(),
                20,
                8,
                "quux1",
                "quux2",
                "quux3",
                "quux4",
                ParsedDateSet::build(Vec::new()).unwrap(),
            )
            .unwrap(),
        );

        assert_eq!(
            MetadataMap::build(data).unwrap().to_json().unwrap(),
            r#"{
    "bar": {
        "lang": "quux1",
        "parsed-dates": [],
        "poetry-lines": 8,
        "repo": "quux3",
        "source-hash": "quux4",
        "version": "quux2",
        "wordcount": 20
    },
    "foo": {
        "lang": "blah1",
        "parsed-dates": [],
        "poetry-lines": 5,
        "repo": "blah3",
        "source-hash": "blah4",
        "version": "blah2",
        "wordcount": 10
    }
}"#,
        )
    }

    #[test]
    fn test_deserialization() {
        let mm: MetadataMap = from_str(
            r#"{
    "bar": {
        "lang": "quux1",
        "parsed-dates": [],
        "poetry-lines": 8,
        "repo": "quux3",
        "source-hash": "quux4",
        "version": "quux2",
        "wordcount": 20
    },
    "foo": {
        "lang": "blah1",
        "parsed-dates": [],
        "poetry-lines": 5,
        "repo": "blah3",
        "source-hash": "blah4",
        "version": "blah2",
        "wordcount": 10
    }
}"#,
        )
        .unwrap();

        let m = &mm.data()["foo"];
        assert!(m.title().is_none());
        assert!(m.authors().is_empty());
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
        assert_eq!(m.wordcount(), 10);
        assert_eq!(m.poetry_lines(), 5);
        assert_eq!(m.lang(), "blah1");
        assert_eq!(m.version(), "blah2");
        assert_eq!(m.repo(), "blah3");
        assert_eq!(m.source_hash(), "blah4");
        assert!(m.parsed_dates().dates().is_empty());

        let m = &mm.data()["bar"];
        assert!(m.title().is_none());
        assert!(m.authors().is_empty());
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
        assert_eq!(m.wordcount(), 20);
        assert_eq!(m.poetry_lines(), 8);
        assert_eq!(m.lang(), "quux1");
        assert_eq!(m.version(), "quux2");
        assert_eq!(m.repo(), "quux3");
        assert_eq!(m.source_hash(), "quux4");
        assert!(m.parsed_dates().dates().is_empty());
    }
}
