use chrono::NaiveDateTime;
use serde::Serialize;

use super::time;

const REGION_AREA_ALL: &[RegionArea] = &[
    RegionArea::Middle,
    RegionArea::North,
    RegionArea::East,
    RegionArea::South,
    RegionArea::SouthEast,
    RegionArea::SouthWest,
    RegionArea::West,
];

#[derive(Clone, Serialize)]
pub struct DisruptionsFilter {
    #[serde(rename = "baustellenAktiv")]
    pub construction_sites: bool,
    #[serde(rename = "baustellenNurTotalsperrung")]
    pub construction_sites_only_full_closures: bool,
    #[serde(rename = "betriebsstellen")]
    pub stations: Vec<String>,
    #[serde(rename = "regionalbereiche")]
    pub region_areas: Vec<RegionArea>,
    #[serde(rename = "stoerungenAktiv")]
    pub disruptions: bool,
    #[serde(rename = "streckennummern")]
    pub railway_line_numbers: Vec<u32>,
    #[serde(rename = "streckenruhenAktiv")]
    pub railway_line_rest: bool,
    #[serde(rename = "wirkungsdauer")]
    pub duration_of_effect: u32,
    #[serde(rename = "zeitraum")]
    pub time: DisruptionsFilterTime,
}

impl Default for DisruptionsFilter {
    fn default() -> Self {
        Self {
            construction_sites: true,
            construction_sites_only_full_closures: false,
            stations: Vec::new(),
            region_areas: Vec::from(REGION_AREA_ALL),
            disruptions: true,
            railway_line_numbers: Vec::new(),
            railway_line_rest: true,
            duration_of_effect: 0,
            time: DisruptionsFilterTime::Hours { hours: 2 },
        }
    }
}

#[derive(Clone, Serialize)]
pub enum RegionArea {
    #[serde(rename = "MITTE")]
    Middle,
    #[serde(rename = "NORD")]
    North,
    #[serde(rename = "OST")]
    East,
    #[serde(rename = "SUED")]
    South,
    #[serde(rename = "SUEDOST")]
    SouthEast,
    #[serde(rename = "SUEDWEST")]
    SouthWest,
    #[serde(rename = "WEST")]
    West,
}

#[derive(Clone, Serialize)]
pub enum DisruptionsFilterTimeWeekday {
    #[serde(rename = "MONTAG")]
    Monday,
    #[serde(rename = "DIENSTAG")]
    Tuesday,
    #[serde(rename = "MITTWOCH")]
    Wednesday,
    #[serde(rename = "DONNERSTAG")]
    Thursday,
    #[serde(rename = "FREITAG")]
    Friday,
    #[serde(rename = "SAMSTAG")]
    Saturday,
    #[serde(rename = "SONNTAG")]
    Sunday,
}

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum DisruptionsFilterTime {
    #[serde(rename = "ROLLIEREND")]
    Hours {
        #[serde(rename = "stunden")]
        hours: u32,
    },
    #[serde(rename = "FIX")]
    Fix {
        #[serde(rename = "beginn", serialize_with = "time::serialize_datetime")]
        start: NaiveDateTime,
        #[serde(rename = "ende", serialize_with = "time::serialize_datetime")]
        end: NaiveDateTime,
        #[serde(rename = "wochentage")]
        weekdays: Vec<DisruptionsFilterTimeWeekday>,
    },
}
