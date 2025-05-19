use serde::{Deserialize, Deserializer, de};
use std::fmt;

pub fn option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }
    Ok(Some(s))
}

pub fn str_or_seq<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StrSeqVisitor;

    impl<'de> de::Visitor<'de> for StrSeqVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a sequence of strings")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![String::from(v)])
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StrSeqVisitor)
}

pub fn uint_or_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct UintStrVisitor;

    impl de::Visitor<'_> for UintStrVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an unsigned int or a string containing an unsigned int")
        }

        fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match val.try_into() {
                Ok(val) => Ok(val),
                Err(_) => Err(E::custom("invalid unsigned int value")),
            }
        }

        fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match val.parse() {
                Ok(val) => self.visit_u64(val),
                Err(_) => Err(E::custom("must contin an unsigned int")),
            }
        }
    }

    deserializer.deserialize_any(UintStrVisitor)
}
