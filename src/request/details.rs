use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::StreckenInfoError;

use super::{request_strecken_info, Disruption, RequestType, ResponseType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DetailsRequest {
    input: String,
    get_trains: bool,
    date: String,
    time: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DetailsResponse {
    pub common: DetailsCommon,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DetailsCommon {
    #[serde(alias = "himL")]
    pub disruptions: Vec<Disruption>,
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
    if let ResponseType::HimDetails { res, err } = response
        .response
        .into_iter()
        .find(|x| matches!(x, ResponseType::HimDetails { .. }))
        .ok_or(StreckenInfoError::InvalidResponse)?
    {
        if err.as_str() != "OK" {
            Err(StreckenInfoError::ResponseError(err))
        } else {
            Ok(res.common.disruptions.into_iter().next())
        }
    } else {
        Err(StreckenInfoError::InvalidResponse)
    }
}
