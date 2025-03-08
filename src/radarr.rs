use reqwest;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

use crate::jellyfin;
use crate::ServiceConfig;

pub async fn queuery_radarr(radarr_config: ServiceConfig, jellyfin_config: ServiceConfig) {
    let url = format!(
        "{}/api/v3/queue?apiKey={}",
        radarr_config.url, radarr_config.api_key
    );
    let mut last_total_records = None;

    loop {
        match fetch_data_radarr(&url).await {
            Ok(response) => {
                if let Some(last) = last_total_records {
                    if response.totalRecords < last {
                        println!("Queue Articles Radarr: {}", response.totalRecords);
                        let _ = jellyfin::trigger_jellyfin_refresh(&jellyfin_config).await;
                    }
                }
                last_total_records = Some(response.totalRecords);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        sleep(Duration::from_millis(500)).await;
    }
}

// Fetch Radarr's API
async fn fetch_data_radarr(url: &str) -> Result<ApiFieldsRadarr, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<ApiFieldsRadarr>().await?;
    Ok(response)
}

// Struct JSON Response
#[derive(Debug, Deserialize)]
struct ApiFieldsRadarr {
    totalRecords: i32,
}
