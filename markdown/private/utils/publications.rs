use crate::{deserializers, json::JsonSerializable, validators};
use chrono::naive::NaiveDate;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum State {
    // Intermediate
    Submitted,
    Accepted,
    // Bad end states
    Abandoned,
    Withdrawn,
    Rejected,
    // Good end states
    SelfPublished,
    Published,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Date {
    pub state: State,
    pub date: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, Builder)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[validate(schema(function = "Publication::validate_contents"))]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct Publication {
    #[validate(custom(function = "validators::non_empty"))]
    venue: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(custom(function = "validators::each_non_empty"))]
    #[builder(default)]
    urls: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "validators::non_empty"))]
    #[builder(default)]
    notes: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserializers::option_string")]
    #[validate(custom(function = "validators::non_empty"))]
    #[builder(default)]
    paid: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    submitted: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    accepted: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    abandoned: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    withdrawn: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    rejected: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    self_published: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    published: Option<NaiveDate>,
}

impl PublicationBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Err(err) = Publication::build(
            self.venue.clone().unwrap_or_default(),
            self.urls.clone().unwrap_or_default(),
            self.notes.clone().unwrap_or_default().as_deref(),
            self.paid.clone().unwrap_or_default().as_deref(),
            self.submitted.unwrap_or_default(),
            self.accepted.unwrap_or_default(),
            self.abandoned.unwrap_or_default(),
            self.withdrawn.unwrap_or_default(),
            self.rejected.unwrap_or_default(),
            self.self_published.unwrap_or_default(),
            self.published.unwrap_or_default(),
        ) {
            return Err(err.to_string());
        }

        Ok(())
    }
}

impl Publication {
    #[allow(clippy::too_many_arguments)]
    fn build<S: Into<String>, V: Into<Vec<String>>>(
        venue: S,
        urls: V,
        notes: Option<&str>,
        paid: Option<&str>,
        submitted: Option<NaiveDate>,
        accepted: Option<NaiveDate>,
        abandoned: Option<NaiveDate>,
        withdrawn: Option<NaiveDate>,
        rejected: Option<NaiveDate>,
        self_published: Option<NaiveDate>,
        published: Option<NaiveDate>,
    ) -> Result<Publication, ValidationErrors> {
        let p = Publication {
            venue: venue.into(),
            urls: urls.into(),
            notes: notes.map(str::to_string),
            paid: paid.map(str::to_string),
            submitted,
            accepted,
            abandoned,
            withdrawn,
            rejected,
            self_published,
            published,
        };
        p.validate()?;
        Ok(p)
    }

    pub fn venue(&self) -> &str {
        &self.venue
    }

    pub fn urls(&self) -> &Vec<String> {
        &self.urls
    }

    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    pub fn paid(&self) -> Option<&String> {
        self.paid.as_ref()
    }

    pub fn submitted(&self) -> Option<&NaiveDate> {
        self.submitted.as_ref()
    }

    pub fn accepted(&self) -> Option<&NaiveDate> {
        self.accepted.as_ref()
    }

    pub fn abandoned(&self) -> Option<&NaiveDate> {
        self.abandoned.as_ref()
    }

    pub fn withdrawn(&self) -> Option<&NaiveDate> {
        self.withdrawn.as_ref()
    }

    pub fn rejected(&self) -> Option<&NaiveDate> {
        self.rejected.as_ref()
    }

    pub fn self_published(&self) -> Option<&NaiveDate> {
        self.self_published.as_ref()
    }

    pub fn published(&self) -> Option<&NaiveDate> {
        self.published.as_ref()
    }

    pub fn dates(&self) -> Vec<Date> {
        self.all_dates().into_iter().flatten().collect()
    }

    pub fn latest(&self) -> Date {
        if let Some(d) = self.dates().into_iter().last() {
            return d;
        }
        panic!("validation should ensure this never happens");
    }

    pub fn active(&self) -> bool {
        self.bad_end_dates().iter().flatten().count() == 0
    }

    fn bad_end_dates(&self) -> Vec<Option<Date>> {
        Self::filter_dates(&[
            (State::Abandoned, self.abandoned()),
            (State::Withdrawn, self.withdrawn()),
            (State::Rejected, self.rejected()),
        ])
    }

    fn good_end_dates(&self) -> Vec<Option<Date>> {
        Self::filter_dates(&[
            (State::SelfPublished, self.self_published()),
            (State::Published, self.published()),
        ])
    }

    fn end_dates(&self) -> Vec<Option<Date>> {
        let mut out = self.bad_end_dates();
        out.append(&mut self.good_end_dates());
        out
    }

    fn intermediate_dates(&self) -> Vec<Option<Date>> {
        Self::filter_dates(&[
            (State::Submitted, self.submitted()),
            (State::Accepted, self.accepted()),
        ])
    }

    fn all_dates(&self) -> Vec<Option<Date>> {
        let mut out = self.intermediate_dates();
        out.append(&mut self.end_dates());
        out
    }

    fn filter_dates(dates: &[(State, Option<&NaiveDate>)]) -> Vec<Option<Date>> {
        let mut out = Vec::new();
        for (s, d) in dates {
            if let Some(&d) = d {
                out.push(Some(Date { state: *s, date: d }));
            } else {
                out.push(None);
            }
        }
        out
    }

    fn validate_contents(&self) -> Result<(), ValidationError> {
        if self.all_dates().iter().flatten().count() == 0 {
            return Err(ValidationError::new("at least one date must be set"));
        }

        if self.end_dates().iter().flatten().count() > 1 {
            return Err(ValidationError::new("at most one end date can be set"));
        }

        if self.self_published.is_some() && self.intermediate_dates().iter().flatten().count() > 0 {
            return Err(ValidationError::new(
                "intermediate dates cannot be used with self_published",
            ));
        }

        if self.accepted.is_some() && self.bad_end_dates().iter().flatten().count() > 0 {
            return Err(ValidationError::new(
                "bad end dates cannot be used with accepted",
            ));
        }

        if self.published.is_some()
            && self
                .intermediate_dates()
                .into_iter()
                .filter(Option::is_none)
                .count()
                > 0
        {
            return Err(ValidationError::new(
                "all intermediate dates must be set when published is set",
            ));
        }

        if self.bad_end_dates().iter().flatten().count() > 0 && self.submitted.is_none() {
            return Err(ValidationError::new(
                "submitted must be set if any bad end dates are set",
            ));
        }

        let dates: Vec<NaiveDate> = self
            .all_dates()
            .into_iter()
            .flatten()
            .map(|d| d.date)
            .collect();
        let mut sorted = dates.clone();
        sorted.sort();
        if dates != sorted {
            return Err(ValidationError::new("dates must be in increasing order"));
        }

        Ok(())
    }
}

impl JsonSerializable for Publication {}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct Publications {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(nested)]
    publications: Vec<Publication>,
}

impl Publications {
    pub fn build<T: Into<Vec<Publication>>>(
        publications: T,
    ) -> Result<Publications, ValidationErrors> {
        let ps = Publications {
            publications: publications.into(),
        };
        ps.validate()?;
        Ok(ps)
    }

    pub fn publications(&self) -> &Vec<Publication> {
        &self.publications
    }

    pub fn is_empty(&self) -> bool {
        self.publications.is_empty()
    }

    pub fn active(&self) -> bool {
        self.publications.iter().any(Publication::active)
    }

    pub fn highest_active_state(&self) -> Option<State> {
        let mut states: Vec<State> = self
            .publications
            .iter()
            .filter(|p| p.active())
            .map(|p| p.latest().state)
            .collect();
        states.sort();
        if let Some(&s) = states.last() {
            return Some(s);
        }
        None
    }
}

impl JsonSerializable for Publications {}

#[cfg(test)]
mod test_utils {
    use super::{Date, State};
    use chrono::NaiveDate;

    pub fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    pub fn date(state: State, date: (i32, u32, u32)) -> Date {
        let (year, month, day) = date;
        Date {
            state,
            date: ymd(year, month, day),
        }
    }
}
#[cfg(test)]
mod publication_test {
    use super::{
        test_utils::{date, ymd},
        Publication, PublicationBuilder, State,
    };
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_good_minimal() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .submitted(ymd(2023, 5, 16))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert!(p.urls().is_empty());
        assert!(p.notes().is_none());
        assert!(p.paid().is_none());
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.dates(), [date(State::Submitted, (2023, 5, 16))]);
        assert_eq!(p.latest(), date(State::Submitted, (2023, 5, 16)));
        assert!(p.active());
    }

    #[test]
    fn test_good_submitted() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.dates(), [date(State::Submitted, (2023, 5, 16))]);
        assert_eq!(p.latest(), date(State::Submitted, (2023, 5, 16)));
        assert!(p.active());
    }

    #[test]
    fn test_good_accepted() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .accepted(ymd(2023, 5, 17))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(
            p.dates(),
            [
                date(State::Submitted, (2023, 5, 16)),
                date(State::Accepted, (2023, 5, 17))
            ]
        );
        assert_eq!(p.latest(), date(State::Accepted, (2023, 5, 17)));
        assert!(p.active());
    }

    #[test]
    fn test_good_rejected() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .rejected(ymd(2023, 5, 17))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert_eq!(p.rejected().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(
            p.dates(),
            [
                date(State::Submitted, (2023, 5, 16)),
                date(State::Rejected, (2023, 5, 17))
            ]
        );
        assert_eq!(p.latest(), date(State::Rejected, (2023, 5, 17)));
        assert!(!p.active());
    }

    #[test]
    fn test_good_withdrawn() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .withdrawn(ymd(2023, 5, 17))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert_eq!(p.withdrawn().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(
            p.dates(),
            [
                date(State::Submitted, (2023, 5, 16)),
                date(State::Withdrawn, (2023, 5, 17))
            ]
        );
        assert_eq!(p.latest(), date(State::Withdrawn, (2023, 5, 17)));
        assert!(!p.active());
    }

    #[test]
    fn test_good_abandoned() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .abandoned(ymd(2023, 5, 17))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert_eq!(p.abandoned().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(
            p.dates(),
            [
                date(State::Submitted, (2023, 5, 16)),
                date(State::Abandoned, (2023, 5, 17))
            ]
        );
        assert_eq!(p.latest(), date(State::Abandoned, (2023, 5, 17)));
        assert!(!p.active());
    }

    #[test]
    fn test_good_self_published() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .self_published(ymd(2023, 5, 16))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert!(p.submitted().is_none());
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert_eq!(p.self_published().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.published().is_none());
        assert_eq!(p.dates(), [date(State::SelfPublished, (2023, 5, 16)),]);
        assert_eq!(p.latest(), date(State::SelfPublished, (2023, 5, 16)));
        assert!(p.active());
    }

    #[test]
    fn test_good_published() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .accepted(ymd(2023, 5, 17))
            .published(ymd(2023, 5, 18))
            .build()
            .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied().unwrap(), ymd(2023, 5, 18));
        assert_eq!(
            p.dates(),
            [
                date(State::Submitted, (2023, 5, 16)),
                date(State::Accepted, (2023, 5, 17)),
                date(State::Published, (2023, 5, 18))
            ]
        );
        assert_eq!(p.latest(), date(State::Published, (2023, 5, 18)));
        assert!(p.active());
    }

    #[test]
    fn test_bad_no_venue() {
        assert!(PublicationBuilder::default()
            .submitted(ymd(2023, 5, 16))
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_no_dates() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_too_many_end_dates() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .rejected(ymd(2023, 5, 16))
            .published(ymd(2023, 5, 16))
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_self_published_intermediate() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .self_published(ymd(2023, 5, 17))
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_accepted_bad_end_dates() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .accepted(ymd(2023, 5, 16))
            .rejected(ymd(2023, 5, 16))
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_published_missing_intermediate() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .published(ymd(2023, 5, 16))
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_bad_missing_submitted() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .rejected(ymd(2023, 5, 16))
            .build()
            .is_err());
    }

    #[test]
    fn test_bad_wrong_order() {
        assert!(PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("foo"), String::from("bar")])
            .notes("baz")
            .paid("quux")
            .submitted(ymd(2023, 5, 16))
            .accepted(ymd(2023, 5, 16))
            .published(ymd(2023, 5, 15))
            .build()
            .is_err());
    }

    #[test]
    fn test_serialization_minimal() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .self_published(ymd(2023, 5, 16))
            .build()
            .unwrap();
        assert_eq!(
            p.to_json().unwrap(),
            r#"{
    "self-published": "2023-05-16",
    "venue": "foo"
}"#
        );
    }

    #[test]
    fn test_serialization_full() {
        let p = PublicationBuilder::default()
            .venue("foo")
            .urls([String::from("bar"), String::from("baz")])
            .notes("quux")
            .paid("blah")
            .submitted(ymd(2023, 5, 16))
            .accepted(ymd(2023, 5, 17))
            .published(ymd(2023, 5, 18))
            .build()
            .unwrap();
        assert_eq!(
            p.to_json().unwrap(),
            r#"{
    "accepted": "2023-05-17",
    "notes": "quux",
    "paid": "blah",
    "published": "2023-05-18",
    "submitted": "2023-05-16",
    "urls": [
        "bar",
        "baz"
    ],
    "venue": "foo"
}"#
        );
    }

    #[test]
    fn test_deserialization_good_minimal() {
        let p: Publication = from_json(
            r#"{
    "notes": "",
    "submitted": "2023-05-16",
    "venue": "foo"
}"#,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert!(p.urls().is_empty());
        assert!(p.notes().is_none());
        assert!(p.paid().is_none());
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.dates(), [date(State::Submitted, (2023, 5, 16))]);
        assert_eq!(p.latest(), date(State::Submitted, (2023, 5, 16)));
        assert!(p.active());
    }

    #[test]
    fn test_deserialization_good_complex() {
        let p: Publication = from_json(
            r#"{
    "accepted": "2023-05-17",
    "notes": "baz",
    "paid": "quux",
    "published": "2023-05-18",
    "submitted": "2023-05-16",
    "urls": [
        "foo",
        "bar"
    ],
    "venue": "foo"
}"#,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied().unwrap(), ymd(2023, 5, 18));
        assert_eq!(
            p.dates(),
            [
                date(State::Submitted, (2023, 5, 16)),
                date(State::Accepted, (2023, 5, 17)),
                date(State::Published, (2023, 5, 18))
            ]
        );
        assert_eq!(p.latest(), date(State::Published, (2023, 5, 18)));
        assert!(p.active());
    }

    #[test]
    fn test_deserialization_bad_no_venue() {
        assert!(from_json::<Publication>(r#"{"submitted": "2023-05-16"}"#).is_err());
    }

    #[test]
    fn test_deserialization_bad_too_many_end_dates() {
        assert!(from_json::<Publication>(
            r#"{
    "published": "2023-05-16",
    "rejected": "2023-05-16",
    "venue": "foo"
}"#,
        )
        .is_err());
    }
}

#[cfg(test)]
mod publications_test {
    use super::{test_utils::ymd, PublicationBuilder, Publications, State};
    use crate::json::{from_json, JsonSerializable};

    #[test]
    fn test_good_active() {
        let ps = Publications::build([
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
        .unwrap();

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
    }

    #[test]
    fn test_good_inactive() {
        let ps = Publications::build([
            PublicationBuilder::default()
                .venue("Book")
                .urls([String::from("foo"), String::from("bar")])
                .notes("baz")
                .paid("quux")
                .submitted(ymd(2023, 5, 16))
                .rejected(ymd(2023, 5, 17))
                .build()
                .unwrap(),
            PublicationBuilder::default()
                .venue("Book2")
                .urls([String::from("foo2"), String::from("bar2")])
                .notes("baz2")
                .paid("quux2")
                .submitted(ymd(2023, 5, 19))
                .withdrawn(ymd(2023, 5, 20))
                .build()
                .unwrap(),
        ])
        .unwrap();

        assert_eq!(ps.publications().len(), 2);
        assert!(!ps.active());
        assert_eq!(ps.highest_active_state(), None);

        let p = &ps.publications()[0];
        assert_eq!(p.venue(), "Book");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert_eq!(p.rejected().copied().unwrap(), ymd(2023, 5, 17));
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");

        let p = &ps.publications()[1];
        assert_eq!(p.venue(), "Book2");
        assert_eq!(p.submitted().copied().unwrap(), ymd(2023, 5, 19));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert_eq!(p.withdrawn().copied().unwrap(), ymd(2023, 5, 20));
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(*p.urls(), &["foo2", "bar2"]);
        assert_eq!(p.notes().unwrap(), "baz2");
        assert_eq!(p.paid().unwrap(), "quux2");
    }

    #[test]
    fn test_empty() {
        let ps = Publications::default();
        assert!(ps.publications().is_empty());
        assert!(!ps.active());
        assert!(ps.highest_active_state().is_none());
    }

    #[test]
    fn test_serialization() {
        let ps = Publications::build([
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
        .unwrap();

        assert_eq!(
            ps.to_json().unwrap(),
            r#"[
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
]"#
        );
    }

    #[test]
    fn test_deserialization_good() {
        let ps: Publications = from_json(
            r#"[
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
]"#,
        )
        .unwrap();

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
    }

    #[test]
    fn test_deserialization_bad_nested() {
        assert!(from_json::<Publications>(r#"[{"venue": "foo"}]"#).is_err());
    }
}
