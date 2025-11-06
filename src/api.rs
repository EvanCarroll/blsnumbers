use serde::{Deserialize, Serialize};
use crate::error::{BlsError, Result};

const BLS_API_V2_URL: &str = "https://api.bls.gov/publicAPI/v2/timeseries/data/";

#[derive(Debug, Serialize)]
pub struct BlsRequest {
    pub seriesid: Vec<String>,
    pub startyear: String,
    pub endyear: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrationkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculations: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BlsResponse {
    pub status: String,
    pub message: Vec<String>,
    #[serde(rename = "Results")]
    pub results: Option<BlsResults>,
}

#[derive(Debug, Deserialize)]
pub struct BlsResults {
    pub series: Vec<BlsSeries>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlsSeries {
    #[serde(rename = "seriesID")]
    pub series_id: String,
    pub data: Vec<BlsDataPoint>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlsDataPoint {
    pub year: String,
    pub period: String,
    #[serde(rename = "periodName")]
    pub period_name: String,
    pub value: String,
    #[allow(dead_code)]
    pub footnotes: Vec<serde_json::Value>,
}

pub struct BlsClient {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl BlsClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn fetch_series(
        &self,
        series_ids: Vec<String>,
        start_year: u16,
        end_year: u16,
    ) -> Result<BlsResponse> {
        let request = BlsRequest {
            seriesid: series_ids,
            startyear: start_year.to_string(),
            endyear: end_year.to_string(),
            registrationkey: self.api_key.clone(),
            calculations: Some(false),
        };

        let response = self
            .client
            .post(BLS_API_V2_URL)
            .json(&request)
            .send()
            .await?;

        let bls_response: BlsResponse = response.json().await?;

        if bls_response.status != "REQUEST_SUCCEEDED" {
            return Err(BlsError::Api(bls_response.message.join("; ")));
        }

        Ok(bls_response)
    }
}
