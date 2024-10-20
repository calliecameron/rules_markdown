use serde::{Deserialize, Serialize, Serializer};
use serde_json_fmt::JsonFormat;
use std::error::Error;
use std::fs::write;
use std::path::Path;
use validator::Validate;

fn sort_alphabetically<T: Serialize, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let value = serde_json::to_value(value).map_err(serde::ser::Error::custom)?;
    value.serialize(serializer)
}

#[derive(Serialize)]
struct SortAlphabetically<T: Serialize>(#[serde(serialize_with = "sort_alphabetically")] T);

pub trait Json {
    fn to_json(&self) -> Result<String, serde_json::Error>
    where
        Self: Serialize,
    {
        let s = JsonFormat::pretty().indent_width(Some(4)).ascii(true);
        let out = s.format_to_string(&SortAlphabetically(self))?;
        Ok(out)
    }

    fn write<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
        Self: Serialize,
    {
        let out = self.to_json()?;
        write(path, out)?;
        Ok(())
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'a> + Validate,
{
    let out: T = serde_json::from_str(s)?;
    out.validate()?;
    Ok(out)
}

pub fn is_false(b: &bool) -> bool {
    !b
}

pub mod deserialize {
    use serde::{de, Deserialize};
    use std::fmt;

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

    pub fn int_or_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct IntStrVisitor;

        impl<'de> de::Visitor<'de> for IntStrVisitor {
            type Value = u32;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an int or a string containing an int")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val.try_into() {
                    Ok(val) => Ok(val),
                    Err(_) => Err(E::custom("invalid int value")),
                }
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val.parse() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Err(E::custom("must contin an int")),
                }
            }
        }

        deserializer.deserialize_any(IntStrVisitor)
    }
}
