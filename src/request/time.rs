use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serializer};

const DB_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveDateTime::parse_from_str(String::deserialize(deserializer)?.as_str(), DB_FORMAT)
        .map_err(de::Error::custom)
}

pub fn serialize_datetime<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.format(DB_FORMAT).to_string())
}
