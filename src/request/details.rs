//! # running `HimDetails` requests
//! ```no_run
//! use strecken_info::details::request_disruption_details;
//! use chrono::Utc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let now = Utc::now().naive_local();
//!     let disruption = request_disruption_details(
//!         "HIM_FREITEXT_SOME_ID",
//!         true, // choose if you want to get the affected trains of the disruption
//!         now
//!     ).await.unwrap().unwrap(); // You need two unwraps, because the Disruption is sent in an `Option` in a `Result`
//! }
//! ```

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{error::StreckenInfoError, Disruption};

use super::{request_strecken_info, RequestType, ResponseType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DetailsRequest {
    input: String,
    get_trains: bool,
    date: String,
    time: String,
}

pub async fn request_disruption_details(
    himid: &str,
    get_trains: bool,
    date: NaiveDateTime,
) -> Result<Option<Disruption>, StreckenInfoError> {
    let request = DetailsRequest {
        input: himid.to_string(),
        get_trains,
        date: date.format("%Y%m%d").to_string(),
        time: date.format("%H%M%S").to_string(),
    };
    let response = request_strecken_info(RequestType::HimDetails { req: request }).await?;
    if let ResponseType::HimDetails { mut res, err } = response
        .response
        .into_iter()
        .find(|x| matches!(x, ResponseType::HimDetails { .. }))
        .ok_or(StreckenInfoError::InvalidResponse)?
    {
        if err.as_str() != "OK" {
            Err(StreckenInfoError::ResponseError(err))
        } else {
            res.common.link()?;
            Ok(res.common.disruptions.into_iter().next())
        }
    } else {
        Err(StreckenInfoError::InvalidResponse)
    }
}
