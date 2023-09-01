use reqwest::{Error, header};
use reqwest;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::Client;

pub type Statuses = Vec<StatusResponse>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    #[serde(rename = "AppID ")]
    pub app_id: String,
    #[serde(rename = "MDM")]
    pub mdm: String,
    #[serde(rename = "SpaceID")]
    pub space_id: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Maintenance")]
    pub maintenance: Value,
    #[serde(rename = "ImpactedFeatures")]
    pub impacted_features: Vec<String>,
}

pub async fn check_status() -> Result<Statuses, Error> {
    let client = Client::new();
    let uri = "https://game-status-api.ubisoft.com/v1/instances?appIds=e3d5ea9e-50bd-43b7-88bf-39794f4e3d40,fb4cc4c9-2063-461d-a1e8-84a7d36525fc,4008612d-3baf-49e4-957a-33066726a7bc";
    let res: Statuses = client.get(uri)
        .header(header::ACCEPT, "application/json")
        .header(header::CACHE_CONTROL, "max-age=0")
        .header(header::CONNECTION, "keep-alive")
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
        .send()
        .await?
        .json::<Statuses>()
        .await?;
    Ok(res)
}