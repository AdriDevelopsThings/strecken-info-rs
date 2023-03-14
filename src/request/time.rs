use chrono::{NaiveDate, NaiveTime};
use serde::{de, Deserialize, Deserializer};

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveDate::parse_from_str(String::deserialize(deserializer)?.as_str(), "%Y%m%d")
        .map_err(de::Error::custom)
}

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveTime::parse_from_str(String::deserialize(deserializer)?.as_str(), "%H%M%S")
        .map_err(de::Error::custom)
}
