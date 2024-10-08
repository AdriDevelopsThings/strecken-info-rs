//! Request a list of all disruptions filtered by [`DisruptionsFilter`]
//! ```no_run
//! use strecken_info::revision::get_revision;
//! use strecken_info::filter::DisruptionsFilter;
//! use strecken_info::disruptions::{request_disruptions, Disruption};
//!
//! #[tokio::main]
//! async fn main() {
//!     let revision: u32 = get_revision().await.unwrap();
//!     let disruptions: Vec<Disruption> = request_disruptions(DisruptionsFilter::default(), revision).await.unwrap();
//!     println!("Got {} disruptions", disruptions.len());
//! }
//! ```

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::StreckenInfoError;

use super::{filter::DisruptionsFilter, float, time};

const DISRUPTIONS_API_PATH: &str = "https://strecken-info.de/api/stoerungen";

pub async fn request_disruptions(
    filter: DisruptionsFilter,
    revision: u32,
) -> Result<Vec<Disruption>, StreckenInfoError> {
    let payload = DisruptionRequestPayload { filter, revision };
    let mut disruptions: Vec<Disruption> = reqwest::Client::new()
        .post(DISRUPTIONS_API_PATH)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    for disruption in disruptions.iter_mut() {
        disruption.stations.dedup_by_key(|s| s.name.clone())
    }

    Ok(disruptions)
}

#[derive(Serialize)]
struct DisruptionRequestPayload {
    filter: DisruptionsFilter,
    revision: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Disruption {
    pub key: String,
    pub subcause: Option<String>,
    pub cause: String,
    #[serde(alias = "abgelaufen")]
    pub expired: bool,
    #[serde(alias = "gleisEinschraenkung")]
    pub track_restriction: TrackRestriction,
    pub text: String,
    #[serde(alias = "regionalbereiche")]
    pub region_areas: Vec<String>,
    #[serde(alias = "regionen")]
    pub regions: Vec<String>,
    #[serde(alias = "koordinaten")]
    pub coordinates: Vec<DisruptionCoordinates>,
    #[serde(alias = "betriebsstellen")]
    pub stations: Vec<DisruptionStation>,
    #[serde(alias = "abschnitte", default)]
    pub sections: Vec<DisruptionSection>,
    #[serde(alias = "wirkungenMitVerkehrsarten")]
    pub effects: Vec<DisruptionEffect>,
    #[serde(alias = "zeitraum")]
    pub period: DisruptionPeriod,
    #[serde(alias = "sammelmeldung")]
    pub collective_report: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TrackRestriction {
    #[serde(alias = "SCHWER")]
    Severe,
    #[serde(alias = "LEICHT")]
    Slight,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisruptionPeriod {
    #[serde(
        alias = "beginn",
        serialize_with = "time::serialize_datetime",
        deserialize_with = "time::deserialize_datetime"
    )]
    pub start: NaiveDateTime,
    #[serde(
        alias = "ende",
        serialize_with = "time::serialize_datetime",
        deserialize_with = "time::deserialize_datetime"
    )]
    pub end: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisruptionStation {
    #[serde(alias = "langname")]
    pub name: String,
    pub ril100: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisruptionSection {
    #[serde(alias = "von")]
    pub from: DisruptionStation,
    #[serde(alias = "bis")]
    pub to: DisruptionStation,
    #[serde(alias = "streckennummer")]
    pub track_number: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Product {
    #[serde(rename = "SPFV")]
    LongDistance,
    #[serde(rename = "SPNV")]
    Local,
    #[serde(rename = "SGV")]
    Freight,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisruptionEffect {
    #[serde(alias = "wirkung")]
    pub effect: String,
    #[serde(alias = "verkehrsarten")]
    pub product: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisruptionCoordinates {
    #[serde(deserialize_with = "float::deserialize_nan_float")]
    pub x: f64,
    #[serde(deserialize_with = "float::deserialize_nan_float")]
    pub y: f64,
}
