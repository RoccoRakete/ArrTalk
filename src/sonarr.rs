use reqwest;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

use crate::jellyfin;
use crate::ServiceConfig;

pub async fn queuery_sonarr(sonarr_config: ServiceConfig, jellyfin_config: ServiceConfig) {
    let url = format!(
        "{}/api/v3/queue?apiKey={}",
        sonarr_config.url, sonarr_config.api_key
    );
    let mut last_total_records = None;

    loop {
        match fetch_data_soanrr(&url).await {
            Ok(response) => {
                if let Some(last) = last_total_records {
                    if response.totalRecords < last {
                        println!("Queue Articles Sonarr: {}", response.totalRecords);
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

// Fetch Sonarr's API
async fn fetch_data_soanrr(url: &str) -> Result<ApiFieldsSonarr, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<ApiFieldsSonarr>().await?;
    Ok(response)
}

// Struct JSON Response
#[derive(Debug, Deserialize)]
struct ApiFieldsSonarr {
    totalRecords: i32,
}
