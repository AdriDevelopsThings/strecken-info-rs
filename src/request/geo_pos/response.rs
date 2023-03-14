use crate::request::time;
use chrono::{NaiveDate, NaiveTime};
use serde::{de, Deserialize, Deserializer};

fn true_fn() -> bool {
    true
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeoPosResponse {
    pub common: GeoPosCommon,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeoPosCommon {
    #[serde(alias = "himL")]
    pub disruptions: Vec<Disruption>,
}

fn deserialize_planned_cat<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(de::Error::custom("Invalid value 'cat' in him_l")),
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Disruption {
    pub act: bool,
    #[serde(alias = "cat", deserialize_with = "deserialize_planned_cat")]
    pub planned: bool,
    #[serde(default = "true_fn")]
    pub display_head: bool,
    #[serde(alias = "eDate", deserialize_with = "time::deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(alias = "eTime", deserialize_with = "time::deserialize_time")]
    pub start_time: NaiveTime,
    #[serde(alias = "sDate", deserialize_with = "time::deserialize_date")]
    pub end_date: NaiveDate,
    #[serde(alias = "sTime", deserialize_with = "time::deserialize_time")]
    pub end_time: NaiveTime,
    #[serde(alias = "hid")]
    pub id: String,
    #[serde(alias = "lModDate", deserialize_with = "time::deserialize_date")]
    pub last_modified_date: NaiveDate,
    #[serde(alias = "lModTime", deserialize_with = "time::deserialize_time")]
    pub last_modified_time: NaiveTime,
    pub prio: u8,
    #[serde(alias = "regionRefL")]
    pub affected_regions: Option<Vec<u16>>,
    pub text: Option<String>,
    #[serde(alias = "impactL")]
    pub impact: Option<Vec<DisruptionImpact>>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Product {
    #[serde(rename = "SPFV")]
    LongDistance,
    #[serde(rename = "SPNV")]
    Local,
    #[serde(rename = "SGV")]
    Freight,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DisruptionImpact {
    #[serde(alias = "prodCode")]
    pub product: Product,
    pub prio: u8,
    pub impact: String,
}
