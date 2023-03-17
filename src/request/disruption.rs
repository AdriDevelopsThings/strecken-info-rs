use chrono::{NaiveDate, NaiveTime};
use serde::{de, Deserialize, Deserializer};

use super::time;

fn true_fn() -> bool {
    true
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
    #[serde(alias = "sDate", deserialize_with = "time::deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(alias = "sTime", deserialize_with = "time::deserialize_time")]
    pub start_time: NaiveTime,
    #[serde(alias = "eDate", deserialize_with = "time::deserialize_date")]
    pub end_date: NaiveDate,
    #[serde(alias = "eTime", deserialize_with = "time::deserialize_time")]
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
    pub head: String,
    pub text: Option<String>,
    #[serde(alias = "impactL")]
    pub impact: Option<Vec<DisruptionImpact>>,
    /// This parameter is only available if you sent a `HimDetails` request with the paramter get_trains=true
    #[serde(rename = "affJnyL")]
    pub affected_journeys: Option<Vec<Journey>>,

    // references
    #[serde(default)]
    pub(crate) edge_ref_l: Vec<u16>,
    #[serde(default)]
    pub(crate) region_ref_l: Vec<u16>,

    #[serde(default, skip_deserializing)]
    pub locations: Vec<LocationRange>,
    #[serde(default, skip_deserializing)]
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone)]
/// An object that contains from `Location` and optional to `Location`
pub struct LocationRange {
    pub from: Location,
    pub to: Option<Location>,
}

/// An object like a Betriebsstelle or something like that
#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub lid: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    #[serde(rename = "extId")]
    pub ext_id: String,
}

/// Regions affected by large disruptions
#[derive(Debug, Deserialize, Clone)]
pub struct Region {
    pub name: String,
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

#[derive(Debug, Deserialize, Clone)]
pub struct Journey {
    /// [JourneyInformation] parsed by the jid
    #[serde(rename = "jid")]
    pub journey_information: JourneyInformation,
}

#[derive(Debug, Clone)]
pub struct JourneyInformation {
    pub jid: String,
    /// The train number includes the train type: e.g. "Tfz81681" or "S  37340"
    pub train_number: String,
}

impl<'de> de::Deserialize<'de> for JourneyInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let jid: String = Deserialize::deserialize(deserializer)?;
        let copied_jid = jid.clone();
        let splitted = copied_jid.split('|').collect::<Vec<&str>>();
        assert!(splitted.len() > 1);
        let splitted_second_part = splitted[1].split('#').collect::<Vec<&str>>();
        Ok(Self {
            jid,
            train_number: splitted_second_part[splitted_second_part.len() - 2].to_string(),
        })
    }
}
