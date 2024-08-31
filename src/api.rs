use crate::models::{IssNow, IssPassResponse};
use reqwest::Client;
use anyhow::Result;

const ISS_NOW_URL: &str = "http://api.open-notify.org/iss-now.json";
const ISS_PASS_URL: &str = "http://api.open-notify.org/iss-pass.json";

pub struct IssApi {
    client: Client,
}

impl IssApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    // Get the current position of the ISS
    pub async fn get_current_position(&self) -> Result<IssNow> {
        let response = self.client.get(ISS_NOW_URL).send().await?;
        let iss_now: IssNow = response.json().await?;
        Ok(iss_now)
    }

    // Get the next pass times for the ISS
    pub async fn get_pass_times(&self, lat: f64, lon: f64) -> Result<IssPassResponse> {
        let url = format!("{}?lat={}&lon={}", ISS_PASS_URL, lat, lon);
        let response = self.client.get(&url).send().await?;
        let pass_times: IssPassResponse = response.json().await?;
        Ok(pass_times)
    }
}
