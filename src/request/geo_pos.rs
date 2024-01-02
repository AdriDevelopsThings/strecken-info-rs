//! # running `HimGeoPos` requests
//! ```no_run
//! use strecken_info::geo_pos::request_disruptions;
//! use chrono::Utc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let now = Utc::now().naive_local();
//!     let response = request_disruptions(
//!         now, // from
//!         now, // to
//!         100, // max_num
//!         100, // prio (100 is very unimportant, 1 is very important)
//!         None // You can also choose a region to search in
//!     ).await.unwrap();
//! }
//! ```

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::{error::StreckenInfoError, Disruption, Event, Location, Region};

use super::{request_strecken_info, time, RequestType, ResponseType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeoPosRequest {
    date_b: String,
    date_e: String,
    get_poly_line: bool,
    him_fltr_l: Vec<HimFiltrL>,
    max_num: u16,
    only_him_id: bool,
    prio: u16,
    rect: GeoPosRect,
    time_b: String,
    time_e: String,
}

#[derive(Serialize)]
pub(crate) struct HimFiltrL {
    mode: String,
    r#type: String,
    value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GeoPosRect {
    ll_crd: Pos,
    ur_crd: Pos,
}

#[derive(Serialize)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GeoPosResponse {
    pub common: GeoPosCommon,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct GeoPosCommon {
    #[serde(alias = "himL", default)]
    pub disruptions: Vec<Disruption>,
    #[serde(alias = "locL")]
    locations: Vec<Location>,
    #[serde(alias = "himMsgRegionL", default)]
    regions: Vec<Region>,
    #[serde(alias = "himMsgEdgeL", default)]
    edges: Vec<Edge>,
    #[serde(alias = "himMsgEventL", default)]
    msg_events: Vec<MsgEvent>,
}

impl GeoPosCommon {
    pub(crate) fn link(&mut self) -> Result<(), StreckenInfoError> {
        for disruption in self.disruptions.iter_mut() {
            for edge in &disruption.edge_ref_l {
                let edge = self
                    .edges
                    .get(*edge as usize)
                    .ok_or(StreckenInfoError::ReferenceError)?;
                let from_loc = self
                    .locations
                    .get(edge.f_loc_x as usize)
                    .ok_or(StreckenInfoError::ReferenceError)?
                    .clone();
                let to_loc = match edge.f_loc_y {
                    Some(f_loc_y) => self
                        .locations
                        .get(f_loc_y as usize)
                        .map(|l| Some(l.clone()))
                        .ok_or(StreckenInfoError::ReferenceError),
                    None => Ok(None),
                }?;
                disruption.locations.push(crate::LocationRange {
                    from: from_loc,
                    to: to_loc,
                });
            }

            for region in &disruption.region_ref_l {
                let region = self
                    .regions
                    .get(*region as usize)
                    .ok_or(StreckenInfoError::ReferenceError)?
                    .clone();
                disruption.regions.push(region);
            }

            for event in &disruption.event_ref_l {
                let event = self
                    .msg_events
                    .get(*event as usize)
                    .ok_or(StreckenInfoError::ReferenceError)?;
                disruption.events.push(Event {
                    start_time: event.f_date.and_time(event.f_time),
                    end_time: event.t_date.and_time(event.t_time),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Edge {
    f_loc_x: u16,
    f_loc_y: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MsgEvent {
    #[serde(deserialize_with = "time::deserialize_date")]
    f_date: NaiveDate,
    #[serde(deserialize_with = "time::deserialize_time")]
    f_time: NaiveTime,
    #[serde(deserialize_with = "time::deserialize_date")]
    t_date: NaiveDate,
    #[serde(deserialize_with = "time::deserialize_time")]
    t_time: NaiveTime,
}

/// Request all disruptions listed on strecken.info
/// Set `prio` to `100` to get all items.
/// You can submit a `pos_range` to limit the range of the responded disruptions.
pub async fn request_disruptions(
    start: NaiveDateTime,
    end: NaiveDateTime,
    max_num: u16,
    prio: u16,
    pos_range: Option<(Pos, Pos)>,
) -> Result<Vec<Disruption>, StreckenInfoError> {
    let pos_range = match pos_range {
        Some(s) => s,
        None => (Pos::new(5383300, 48026672), Pos::new(14238281, 54156001)),
    };

    let request = GeoPosRequest {
        date_b: start.format("%Y%m%d").to_string(),
        time_b: start.format("%H%M%S").to_string(),
        date_e: end.format("%Y%m%d").to_string(),
        time_e: end.format("%H%M%S").to_string(),
        get_poly_line: false,
        him_fltr_l: vec![
            HimFiltrL {
                mode: "INC".to_string(),
                r#type: "HIMCAT".to_string(),
                value: "0".to_string(),
            },
            HimFiltrL {
                mode: "INC".to_string(),
                r#type: "HIMCAT".to_string(),
                value: "1".to_string(),
            },
        ],
        max_num,
        only_him_id: false,
        prio,
        rect: GeoPosRect {
            ll_crd: pos_range.0,
            ur_crd: pos_range.1,
        },
    };
    let response = request_strecken_info(RequestType::HimGeoPos { req: request }).await?;
    if let ResponseType::HimGeoPos { mut res, err } = response
        .response
        .into_iter()
        .find(|x| matches!(x, ResponseType::HimGeoPos { .. }))
        .ok_or(StreckenInfoError::InvalidResponse)?
    {
        if err.as_str() != "OK" {
            Err(StreckenInfoError::ResponseError(err))
        } else {
            res.common.link()?;
            Ok(res.common.disruptions)
        }
    } else {
        Err(StreckenInfoError::InvalidResponse)
    }
}
