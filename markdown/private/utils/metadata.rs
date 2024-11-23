use crate::{deserializers, field_validators, json::JsonSerializable, publications::Publications};
use chrono::naive::NaiveDate;
use derive_builder::Builder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashSet;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Version {
    #[validate(custom(function = "field_validators::non_empty"))]
    pub version: String,
    #[validate(custom(function = "field_validators::non_empty"))]
    pub repo: String,
}

impl Version {
    pub fn build<T: Into<String>>(version: T, repo: T) -> Result<Version, ValidationErrors> {
        let v = Version {
            version: version.into(),
            repo: repo.into(),
        };
        v.validate()?;
        Ok(v)
    }
}

impl JsonSerializable for Version {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct SourceHash {
    #[validate(custom(function = "field_validators::non_empty"))]
    pub source_hash: String,
}

impl SourceHash {
    pub fn build<T: Into<String>>(source_hash: T) -> Result<SourceHash, ValidationErrors> {
        let s = SourceHash {
            source_hash: source_hash.into(),
        };
        s.validate()?;
        Ok(s)
    }
}

impl JsonSerializable for SourceHash {}

#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
#[serde(rename_all = "kebab-case")]
pub struct ParsedDateSet {
    #[validate(custom(function = "ParsedDateSet::validate_sorted_date_set"))]
    parsed_dates: Vec<String>,
}

impl ParsedDateSet {
    pub fn build<T: Into<Vec<String>>>(parsed_dates: T) -> Result<ParsedDateSet, ValidationErrors> {
        let pd = ParsedDateSet {
            parsed_dates: parsed_dates.into(),
        };
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

impl JsonSerializable for ParsedDateSet {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Validate)]
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

impl JsonSerializable for ParsedDates {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Identifier {
    #[validate(custom(function = "field_validators::non_empty"))]
    pub scheme: String,
    #[validate(custom(function = "field_validators::non_empty"))]
    pub text: String,
}

impl Identifier {
    pub fn build<T: Into<String>>(scheme: T, text: T) -> Result<Identifier, ValidationErrors> {
        let i = Identifier {
            scheme: scheme.into(),
            text: text.into(),
        };
        i.validate()?;
        Ok(i)
    }
}

impl JsonSerializable for Identifier {}

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Validate, Builder)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct InputMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "field_validators::non_empty"))]
    #[builder(default)]
    title: Option<String>,

    #[serde(default)]
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "deserializers::str_or_seq")]
    #[validate(custom(function = "field_validators::each_non_empty"))]
    #[builder(default)]
    authors: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "field_validators::non_empty"))]
    #[builder(default)]
    date: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "field_validators::non_empty"))]
    #[builder(default)]
    notes: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    #[builder(default)]
    finished: bool,

    #[serde(default)]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Publications::is_empty")]
    #[builder(default)]
    publications: Publications,

    #[serde(default)]
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(nested)]
    #[builder(default)]
    identifiers: Vec<Identifier>,
}

impl InputMetadataBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Err(err) = InputMetadata::build(
            self.title.clone().unwrap_or_default().as_deref(),
            self.authors.clone().unwrap_or_default(),
            self.date.clone().unwrap_or_default().as_deref(),
            self.notes.clone().unwrap_or_default().as_deref(),
            self.finished.unwrap_or_default(),
            self.publications.clone().unwrap_or_default(),
            self.identifiers.clone().unwrap_or_default(),
        ) {
            return Err(err.to_string());
        }

        Ok(())
    }
}

impl InputMetadata {
    fn build<S, I>(
        title: Option<&str>,
        authors: S,
        date: Option<&str>,
        notes: Option<&str>,
        finished: bool,
        publications: Publications,
        identifiers: I,
    ) -> Result<InputMetadata, ValidationErrors>
    where
        S: Into<Vec<String>>,
        I: Into<Vec<Identifier>>,
    {
        let m = InputMetadata {
            title: title.map(str::to_string),
            authors: authors.into(),
            date: date.map(str::to_string),
            notes: notes.map(str::to_string),
            finished,
            publications,
            identifiers: identifiers.into(),
        };
        m.validate()?;
        Ok(m)
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn authors(&self) -> &Vec<String> {
        &self.authors
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
        &self.identifiers
    }
}

impl JsonSerializable for InputMetadata {}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, Builder)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct OutputMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "field_validators::non_empty"))]
    #[builder(default)]
    title: Option<String>,

    #[serde(default)]
    #[serde(rename = "author")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "deserializers::str_or_seq")]
    #[validate(custom(function = "field_validators::each_non_empty"))]
    #[builder(default)]
    authors: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "field_validators::non_empty"))]
    #[builder(default)]
    date: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "field_validators::non_empty"))]
    #[builder(default)]
    notes: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_false")]
    #[builder(default)]
    finished: bool,

    #[serde(default)]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Publications::is_empty")]
    #[builder(default)]
    publications: Publications,

    #[serde(default)]
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(nested)]
    #[builder(default)]
    identifiers: Vec<Identifier>,

    #[serde(deserialize_with = "deserializers::uint_or_str")]
    wordcount: u32,

    #[serde(deserialize_with = "deserializers::uint_or_str")]
    poetry_lines: u32,

    #[validate(custom(function = "field_validators::non_empty"))]
    lang: String,

    #[validate(custom(function = "field_validators::non_empty"))]
    version: String,

    #[validate(custom(function = "field_validators::non_empty"))]
    repo: String,

    #[validate(custom(function = "field_validators::non_empty"))]
    source_hash: String,

    #[serde(default)]
    #[validate(nested)]
    #[builder(default)]
    parsed_dates: ParsedDateSet,
}

impl OutputMetadataBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Err(err) = OutputMetadata::build(
            self.title.clone().unwrap_or_default().as_deref(),
            self.authors.clone().unwrap_or_default(),
            self.date.clone().unwrap_or_default().as_deref(),
            self.notes.clone().unwrap_or_default().as_deref(),
            self.finished.unwrap_or_default(),
            self.publications.clone().unwrap_or_default(),
            self.identifiers.clone().unwrap_or_default(),
            self.wordcount.unwrap_or_default(),
            self.poetry_lines.unwrap_or_default(),
            self.lang.clone().unwrap_or_default(),
            self.version.clone().unwrap_or_default(),
            self.repo.clone().unwrap_or_default(),
            self.source_hash.clone().unwrap_or_default(),
            self.parsed_dates.clone().unwrap_or_default(),
        ) {
            return Err(err.to_string());
        }

        Ok(())
    }
}

impl OutputMetadata {
    #[allow(clippy::too_many_arguments)]
    fn build<VS, VI, S>(
        title: Option<&str>,
        authors: VS,
        date: Option<&str>,
        notes: Option<&str>,
        finished: bool,
        publications: Publications,
        identifiers: VI,
        wordcount: u32,
        poetry_lines: u32,
        lang: S,
        version: S,
        repo: S,
        source_hash: S,
        parsed_dates: ParsedDateSet,
    ) -> Result<OutputMetadata, ValidationErrors>
    where
        VS: Into<Vec<String>>,
        VI: Into<Vec<Identifier>>,
        S: Into<String>,
    {
        let m = OutputMetadata {
            title: title.map(str::to_string),
            authors: authors.into(),
            date: date.map(str::to_string),
            notes: notes.map(str::to_string),
            finished,
            publications,
            identifiers: identifiers.into(),
            wordcount,
            poetry_lines,
            lang: lang.into(),
            version: version.into(),
            repo: repo.into(),
            source_hash: source_hash.into(),
            parsed_dates,
        };
        m.validate()?;
        Ok(m)
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn authors(&self) -> &Vec<String> {
        &self.authors
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
        &self.identifiers
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

impl JsonSerializable for OutputMetadata {}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct MetadataMap {
    #[validate(nested)]
    data: BTreeMap<String, OutputMetadata>,
}

impl MetadataMap {
    pub fn build<T: Into<BTreeMap<String, OutputMetadata>>>(
        data: T,
    ) -> Result<MetadataMap, ValidationErrors> {
        let m = MetadataMap { data: data.into() };
        m.validate()?;
        Ok(m)
    }

    pub fn data(&self) -> &BTreeMap<String, OutputMetadata> {
        &self.data
    }
}

impl JsonSerializable for MetadataMap {}

#[cfg(test)]
mod test_utils {
    use chrono::NaiveDate;

    pub fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

#[cfg(test)]
mod version_test {
    use super::Version;
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_build() {
        assert!(Version::build("", "bar").is_err());
        assert!(Version::build("foo", "").is_err());
        assert!(Version::build("", "").is_err());
    }

    #[test]
    fn test_serialization() {
        assert_eq!(
            Version::build("foo", "bar").unwrap().to_json().unwrap(),
            r#"{
    "repo": "bar",
    "version": "foo"
}"#
        );
    }

    #[test]
    fn test_deserialization() {
        let v: Version = from_json(
            r#"{
    "repo": "bar",
    "version": "foo"
}"#,
        )
        .unwrap();
        assert_eq!(v.version, "foo");
        assert_eq!(v.repo, "bar");

        assert!(from_json::<Version>(
            r#"{
    "repo": "bar",
    "version": ""
}"#
        )
        .is_err());
        assert!(from_json::<Version>(
            r#"{
    "repo": "",
    "version": "foo"
}"#
        )
        .is_err());
        assert!(from_json::<Version>(
            r#"{
    "repo": "",
    "version": ""
}"#
        )
        .is_err());
    }
}

#[cfg(test)]
mod source_hash_test {
    use super::SourceHash;
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_build() {
        assert!(SourceHash::build("").is_err());
    }

    #[test]
    fn test_serialization() {
        assert_eq!(
            SourceHash::build("foo").unwrap().to_json().unwrap(),
            r#"{
    "source-hash": "foo"
}"#
        )
    }

    #[test]
    fn test_deserialization() {
        let h: SourceHash = from_json(
            r#"{
    "source-hash": "foo"
}"#,
        )
        .unwrap();
        assert_eq!(h.source_hash, "foo");

        assert!(from_json::<SourceHash>(
            r#"{
    "source-hash": "",
}"#
        )
        .is_err());
    }
}

#[cfg(test)]
mod parsed_date_set_test {
    use super::ParsedDateSet;

    #[test]
    fn test_bad() {
        assert!(ParsedDateSet::build([String::from("foo")]).is_err());
        assert!(ParsedDateSet::build([String::new()]).is_err());
    }
}

#[cfg(test)]
mod parsed_dates_test {
    use super::{ParsedDateSet, ParsedDates};
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_serialization() {
        assert_eq!(
            ParsedDates::build(
                ParsedDateSet::build([String::from("2020"), String::from("2020/01")]).unwrap()
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
        let pd: ParsedDates = from_json(
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
        assert!(from_json::<ParsedDates>(
            r#"{
    "parsed-dates": [
        "2020/01/01 10:30:00"
    ]
}"#
        )
        .is_err());

        assert!(from_json::<ParsedDates>(
            r#"{
    "parsed-dates": [
        ""
    ]
}"#
        )
        .is_err());
    }

    #[test]
    fn test_deserialization_bad_duplicates() {
        assert!(from_json::<ParsedDates>(
            r#"{
    "parsed-dates": [
        "2020/01/01",
        "2020/01/01"
    ]
}"#,
        )
        .is_err());
    }

    #[test]
    fn test_deserialization_bad_unordered() {
        assert!(from_json::<ParsedDates>(
            r#"{
    "parsed-dates": [
        "2024/06/23",
        "2020/01/01"
    ]
}"#,
        )
        .is_err());
    }
}

#[cfg(test)]
mod identifier_test {
    use super::Identifier;
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_build() {
        assert!(Identifier::build("", "bar").is_err());
        assert!(Identifier::build("foo", "").is_err());
        assert!(Identifier::build("", "").is_err());
    }

    #[test]
    fn test_serialization() {
        assert_eq!(
            Identifier::build("foo", "bar").unwrap().to_json().unwrap(),
            r#"{
    "scheme": "foo",
    "text": "bar"
}"#
        )
    }

    #[test]
    fn test_deserialization() {
        let i: Identifier = from_json(
            r#"{
    "scheme": "foo",
    "text": "bar"
}"#,
        )
        .unwrap();
        assert_eq!(i.scheme, "foo");
        assert_eq!(i.text, "bar");

        assert!(from_json::<Identifier>(
            r#"{
    "scheme": "",
    "text": "bar"
}"#
        )
        .is_err());
        assert!(from_json::<Identifier>(
            r#"{
    "scheme": "foo",
    "text": ""
}"#
        )
        .is_err());
        assert!(from_json::<Identifier>(
            r#"{
    "scheme": "",
    "text": ""
}"#
        )
        .is_err());
    }
}

#[cfg(test)]
mod input_metadata_test {
    use super::{test_utils::ymd, Identifier, InputMetadata, InputMetadataBuilder};
    use crate::json::{from_json, JsonSerializable};
    use crate::publications::{PublicationBuilder, Publications, State};

    #[test]
    fn test_serialization_minimal() {
        assert_eq!(InputMetadata::default().to_json().unwrap(), r#"{}"#)
    }

    #[test]
    fn test_serialization_full() {
        assert_eq!(
            InputMetadataBuilder::default()
                .title("foo")
                .authors([String::from("bar"), String::from("baz")])
                .date("quux")
                .notes("blah")
                .finished(true)
                .publications(
                    Publications::build([
                        PublicationBuilder::default()
                            .venue("Book")
                            .urls([String::from("foo"), String::from("bar")])
                            .notes("baz")
                            .paid("quux")
                            .submitted(ymd(2023, 5, 16))
                            .accepted(ymd(2023, 5, 17))
                            .published(ymd(2023, 5, 18))
                            .build()
                            .unwrap(),
                        PublicationBuilder::default()
                            .venue("Book2")
                            .urls([String::from("foo2"), String::from("bar2")])
                            .notes("baz2")
                            .paid("quux2")
                            .submitted(ymd(2023, 5, 19))
                            .accepted(ymd(2023, 5, 20))
                            .build()
                            .unwrap(),
                    ])
                    .unwrap()
                )
                .identifiers([
                    Identifier::build("a", "b").unwrap(),
                    Identifier::build("c", "d").unwrap()
                ])
                .build()
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
        let m = from_json::<InputMetadata>(r#"{}"#).unwrap();
        assert!(m.title().is_none());
        assert!(m.authors().is_empty());
        assert!(m.date().is_none());
        assert!(m.notes().is_none());
        assert!(!m.finished());
        assert!(m.publications().is_empty());
        assert!(m.identifiers().is_empty());
    }

    #[test]
    fn test_deserialization_conversions() {
        let m = from_json::<InputMetadata>(
            r#"{
    "title": "",
    "date": "",
    "notes": ""
}"#,
        )
        .unwrap();
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
        let m = from_json::<InputMetadata>(r#"{"author": "foo"}"#).unwrap();
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
        let m = from_json::<InputMetadata>(
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
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied().unwrap(), ymd(2023, 5, 18));
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");

        let p = &ps.publications()[1];
        assert_eq!(p.venue(), "Book2");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 19));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 20));
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
            &[
                Identifier::build("a", "b").unwrap(),
                Identifier::build("c", "d").unwrap()
            ]
        );
    }
}

#[cfg(test)]
mod output_metadata_test {
    use super::{
        test_utils::ymd, Identifier, OutputMetadata, OutputMetadataBuilder, ParsedDateSet,
    };
    use crate::json::{from_json, JsonSerializable};
    use crate::publications::{PublicationBuilder, Publications, State};

    #[test]
    fn test_serialization_full() {
        assert_eq!(
            OutputMetadataBuilder::default()
                .title("foo")
                .authors([String::from("bar"), String::from("baz")])
                .date("quux")
                .notes("blah")
                .finished(true)
                .publications(
                    Publications::build([
                        PublicationBuilder::default()
                            .venue("Book")
                            .urls([String::from("foo"), String::from("bar")])
                            .notes("baz")
                            .paid("quux")
                            .submitted(ymd(2023, 5, 16))
                            .accepted(ymd(2023, 5, 17))
                            .published(ymd(2023, 5, 18))
                            .build()
                            .unwrap(),
                        PublicationBuilder::default()
                            .venue("Book2")
                            .urls([String::from("foo2"), String::from("bar2")])
                            .notes("baz2")
                            .paid("quux2")
                            .submitted(ymd(2023, 5, 19))
                            .accepted(ymd(2023, 5, 20))
                            .build()
                            .unwrap(),
                    ])
                    .unwrap()
                )
                .identifiers([
                    Identifier::build("a", "b").unwrap(),
                    Identifier::build("c", "d").unwrap()
                ])
                .wordcount(10u32)
                .poetry_lines(5u32)
                .lang("blah1")
                .version("blah2")
                .repo("blah3")
                .source_hash("blah4")
                .parsed_dates(
                    ParsedDateSet::build([String::from("2020"), String::from("2020/01")]).unwrap()
                )
                .build()
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
        let m = from_json::<OutputMetadata>(
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
        let m = from_json::<OutputMetadata>(
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
        let m = from_json::<OutputMetadata>(
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
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied().unwrap(), ymd(2023, 5, 18));
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");

        let p = &ps.publications()[1];
        assert_eq!(p.venue(), "Book2");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 19));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 20));
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
            &[
                Identifier::build("a", "b").unwrap(),
                Identifier::build("c", "d").unwrap()
            ]
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

    use super::{MetadataMap, OutputMetadataBuilder};
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_serialization() {
        let m = MetadataMap::build(BTreeMap::from([
            (
                String::from("foo"),
                OutputMetadataBuilder::default()
                    .wordcount(10u32)
                    .poetry_lines(5u32)
                    .lang("blah1")
                    .version("blah2")
                    .repo("blah3")
                    .source_hash("blah4")
                    .build()
                    .unwrap(),
            ),
            (
                String::from("bar"),
                OutputMetadataBuilder::default()
                    .wordcount(20u32)
                    .poetry_lines(8u32)
                    .lang("quux1")
                    .version("quux2")
                    .repo("quux3")
                    .source_hash("quux4")
                    .build()
                    .unwrap(),
            ),
        ]))
        .unwrap();

        assert_eq!(
            m.to_json().unwrap(),
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
        let mm = from_json::<MetadataMap>(
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
