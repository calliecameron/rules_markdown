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

pub trait JsonSerializable {
    fn to_json(&self) -> Result<String, serde_json::Error>
    where
        Self: Serialize,
    {
        let s = JsonFormat::pretty().indent_width(Some(4)).ascii(true);
        let out = s.format_to_string(&SortAlphabetically(self))?;
        Ok(out)
    }

    fn write_json<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
        Self: Serialize,
    {
        let out = self.to_json()?;
        write(path, out)?;
        Ok(())
    }
}

pub fn from_json<'a, T>(s: &'a str) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'a> + Validate,
{
    let out: T = serde_json::from_str(s)?;
    out.validate()?;
    Ok(out)
}
