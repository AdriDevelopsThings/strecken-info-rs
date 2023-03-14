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

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::error::StreckenInfoError;

mod response;
pub use response::GeoPosResponse;

use self::response::Disruption;

use super::{request_strecken_info, RequestType, ResponseType};

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
        time_e: end.format("%Y%m%d").to_string(),
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
                value: "1023".to_string(),
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
    let ResponseType::HimGeoPos { res, err } = response
        .response
        .into_iter()
        .find(|x| matches!(x, ResponseType::HimGeoPos { .. }))
        .ok_or(StreckenInfoError::InvalidResponse)?;
    if err.as_str() != "OK" {
        Err(StreckenInfoError::ResponseError(err))
    } else {
        Ok(res.common.disruptions)
    }
}
