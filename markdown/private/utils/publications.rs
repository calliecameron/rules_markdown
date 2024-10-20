use crate::json::Json;
use chrono::naive::NaiveDate;
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

#[derive(Serialize, Deserialize, Default, Validate)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[validate(schema(function = "Publication::validate_contents"))]
pub struct Publication {
    venue: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    urls: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    paid: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    submitted: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    abandoned: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    rejected: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    self_published: Option<NaiveDate>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<NaiveDate>,
}

impl Publication {
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        venue: &str,
        urls: Vec<String>,
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
            venue: String::from(venue),
            urls,
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

impl Json for Publication {}

#[derive(Serialize, Deserialize, Default, Validate)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct Publications {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[validate(nested)]
    publications: Vec<Publication>,
}

impl Publications {
    pub fn build(publications: Vec<Publication>) -> Result<Publications, ValidationErrors> {
        let ps = Publications { publications };
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

impl Json for Publications {}

#[cfg(test)]
mod test_utils {
    use super::{Date, State};
    use chrono::NaiveDate;

    pub fn ymd(year: i32, month: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(year, month, day)
    }

    pub fn date(state: State, date: (i32, u32, u32)) -> Date {
        let (year, month, day) = date;
        Date {
            state,
            date: ymd(year, month, day).unwrap(),
        }
    }
}
#[cfg(test)]
mod publication_test {
    use super::{
        test_utils::{date, ymd},
        Publication, State,
    };
    use crate::json::{from_str, Json};

    #[test]
    fn test_good_minimal() {
        let p = Publication::build(
            "foo",
            Vec::new(),
            None,
            None,
            ymd(2023, 5, 16),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert!(p.urls().is_empty());
        assert!(p.notes().is_none());
        assert!(p.paid().is_none());
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
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
        let p = Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
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
        let p = Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            ymd(2023, 5, 17),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 17));
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
        let p = Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            None,
            None,
            None,
            ymd(2023, 5, 17),
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert_eq!(p.rejected().copied(), ymd(2023, 5, 17));
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
        let p = Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            None,
            None,
            ymd(2023, 5, 17),
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert_eq!(p.withdrawn().copied(), ymd(2023, 5, 17));
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
        let p = Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            None,
            ymd(2023, 5, 17),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert_eq!(p.abandoned().copied(), ymd(2023, 5, 17));
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
        let p = Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            None,
            None,
            None,
            None,
            None,
            ymd(2023, 5, 16),
            None,
        )
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
        assert_eq!(p.self_published().copied(), ymd(2023, 5, 16));
        assert!(p.published().is_none());
        assert_eq!(p.dates(), [date(State::SelfPublished, (2023, 5, 16)),]);
        assert_eq!(p.latest(), date(State::SelfPublished, (2023, 5, 16)));
        assert!(p.active());
    }

    #[test]
    fn test_good_published() {
        let p = Publication::build(
            "foo",
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
        .unwrap();
        assert_eq!(p.venue(), "foo");
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied(), ymd(2023, 5, 18));
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
    fn test_bad_no_dates() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .is_err());
    }

    #[test]
    fn test_bad_too_many_end_dates() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            None,
            None,
            None,
            None,
            ymd(2023, 5, 16),
            None,
            ymd(2023, 5, 16),
        )
        .is_err());
    }

    #[test]
    fn test_bad_self_published_intermediate() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            None,
            None,
            None,
            None,
            ymd(2023, 5, 17),
            None,
        )
        .is_err());
    }

    #[test]
    fn test_bad_accepted_bad_end_dates() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            None,
            ymd(2023, 5, 16),
            None,
            None,
            ymd(2023, 5, 16),
            None,
            None,
        )
        .is_err());
    }

    #[test]
    fn test_bad_published_missing_intermediate() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            None,
            None,
            None,
            None,
            None,
            ymd(2023, 5, 16),
        )
        .is_err());
    }

    #[test]
    fn test_bad_bad_missing_submitted() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            None,
            None,
            None,
            None,
            ymd(2023, 5, 16),
            None,
            None,
        )
        .is_err());
    }

    #[test]
    fn test_bad_wrong_order() {
        assert!(Publication::build(
            "foo",
            vec![String::from("foo"), String::from("bar")],
            Some("baz"),
            Some("quux"),
            ymd(2023, 5, 16),
            ymd(2023, 5, 16),
            None,
            None,
            None,
            None,
            ymd(2023, 5, 15),
        )
        .is_err());
    }

    #[test]
    fn test_serialization_minimal() {
        let p = Publication::build(
            "foo",
            Vec::new(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            ymd(2023, 5, 16),
            None,
        )
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
        let p = Publication::build(
            "foo",
            vec![String::from("bar"), String::from("baz")],
            Some("quux"),
            Some("blah"),
            ymd(2023, 5, 16),
            ymd(2023, 5, 17),
            None,
            None,
            None,
            None,
            ymd(2023, 5, 18),
        )
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
    fn test_deserialization_good() {
        let p: Publication = from_str(
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
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert_eq!(p.accepted().copied(), ymd(2023, 5, 17));
        assert!(p.rejected().is_none());
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert_eq!(p.published().copied(), ymd(2023, 5, 18));
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
        let p: Result<Publication, _> = from_str(r#"{"submitted": "2023-05-16"}"#);
        assert!(p.is_err());
    }

    #[test]
    fn test_deserialization_bad_too_many_end_dates() {
        let p: Result<Publication, _> = from_str(
            r#"{
    "published": "2023-05-16",
    "rejected": "2023-05-16",
    "venue": "foo"
}"#,
        );
        assert!(p.is_err());
    }
}

#[cfg(test)]
mod publications_test {
    use super::{test_utils::ymd, Publication, Publications, State};
    use crate::json::{from_str, Json};

    #[test]
    fn test_good_active() {
        let ps = Publications::build(vec![
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
        .unwrap();

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
    }

    #[test]
    fn test_good_inactive() {
        let ps = Publications::build(vec![
            Publication::build(
                "Book",
                vec![String::from("foo"), String::from("bar")],
                Some("baz"),
                Some("quux"),
                ymd(2023, 5, 16),
                None,
                None,
                None,
                ymd(2023, 5, 17),
                None,
                None,
            )
            .unwrap(),
            Publication::build(
                "Book2",
                vec![String::from("foo2"), String::from("bar2")],
                Some("baz2"),
                Some("quux2"),
                ymd(2023, 5, 19),
                None,
                None,
                ymd(2023, 5, 20),
                None,
                None,
                None,
            )
            .unwrap(),
        ])
        .unwrap();

        assert_eq!(ps.publications().len(), 2);
        assert!(!ps.active());
        assert_eq!(ps.highest_active_state(), None);

        let p = &ps.publications()[0];
        assert_eq!(p.venue(), "Book");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 16));
        assert!(p.accepted().is_none());
        assert_eq!(p.rejected().copied(), ymd(2023, 5, 17));
        assert!(p.withdrawn().is_none());
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(p.urls(), &["foo", "bar"]);
        assert_eq!(p.notes().unwrap(), "baz");
        assert_eq!(p.paid().unwrap(), "quux");

        let p = &ps.publications()[1];
        assert_eq!(p.venue(), "Book2");
        assert_eq!(p.submitted().copied(), ymd(2023, 5, 19));
        assert!(p.accepted().is_none());
        assert!(p.rejected().is_none());
        assert_eq!(p.withdrawn().copied(), ymd(2023, 5, 20));
        assert!(p.abandoned().is_none());
        assert!(p.self_published().is_none());
        assert!(p.published().is_none());
        assert_eq!(*p.urls(), &["foo2", "bar2"]);
        assert_eq!(p.notes().unwrap(), "baz2");
        assert_eq!(p.paid().unwrap(), "quux2");
    }

    #[test]
    fn test_empty() {
        let ps = Publications::build(Vec::new()).unwrap();
        assert!(ps.publications().is_empty());
        assert!(!ps.active());
        assert!(ps.highest_active_state().is_none());
    }

    #[test]
    fn test_serialization() {
        let ps = Publications::build(vec![
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
        let ps: Publications = from_str(
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
    }

    #[test]
    fn test_deserialization_bad_nested() {
        let ps: Result<Publications, _> = from_str(r#"[{"venue": "foo"}]"#);
        assert!(ps.is_err());
    }
}
