use crate::error::StreckenInfoError;
use serde::{Deserialize, Serialize};

use self::{
    details::{DetailsRequest, DetailsResponse},
    geo_pos::{GeoPosRequest, GeoPosResponse},
};

pub mod details;
pub mod disruption;
pub mod geo_pos;
mod time;

#[derive(Serialize)]
struct FullRequest {
    auth: RequestAuth,
    client: RequestClient,
    ext: String,
    formatted: bool,
    lang: String,
    #[serde(rename = "svcReqL")]
    svc_req_l: Vec<SvcRequest>,
    ver: String,
}

#[derive(Serialize)]
struct RequestAuth {
    aid: String,
    r#type: String,
}

#[derive(Serialize)]
struct RequestClient {
    id: String,
    name: String,
    r#type: String,
    v: String,
}

#[derive(Serialize)]
struct SvcRequest {
    cfg: SvcRequestConfig,
    #[serde(flatten)]
    request: RequestType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SvcRequestConfig {
    cfg_grp_l: Vec<String>,
    cfg_hash: String,
}

#[derive(Serialize)]
#[serde(tag = "meth", content = "req")]
pub(crate) enum RequestType {
    HimGeoPos {
        #[serde(flatten)]
        req: GeoPosRequest,
    },
    HimDetails {
        #[serde(flatten)]
        req: DetailsRequest,
    },
}

#[derive(Deserialize)]
pub(crate) struct Response {
    // pub ver: String,
    // pub ext: String,
    // pub lang: String,
    // pub id: String,
    #[serde(rename = "svcResL")]
    pub response: Vec<ResponseType>,
}

#[derive(Deserialize)]
#[serde(tag = "meth")]
pub(crate) enum ResponseType {
    HimGeoPos { res: GeoPosResponse, err: String },
    HimDetails { res: DetailsResponse, err: String },
}

pub(crate) async fn request_strecken_info(
    request: RequestType,
) -> Result<Response, StreckenInfoError> {
    let request = FullRequest {
        auth: RequestAuth {
            aid: "hf7mcf9bv3nv8g5f".to_string(),
            r#type: "AID".to_string(),
        },
        client: RequestClient {
            id: "DBZUGRADARNETZ".to_string(),
            name: "webapp".to_string(),
            r#type: "WEB".to_string(),
            v: "0.1.0".to_string(),
        },
        ext: "DBNETZZUGRADAR.2".to_string(),
        formatted: false,
        lang: "deu".to_string(),
        svc_req_l: vec![SvcRequest {
            cfg: SvcRequestConfig {
                cfg_grp_l: Vec::new(),
                cfg_hash: "i74dckao7PmBwS0rbk0p".to_string(),
            },
            request,
        }],
        ver: "1.15".to_string(),
    };
    Ok(reqwest::Client::new()
        .post("https://db-livemaps.hafas.de/bin/mgate.exe")
        .body(serde_json::to_string(&request)?)
        .send()
        .await?
        .json()
        .await?)
}
