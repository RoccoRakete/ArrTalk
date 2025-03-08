use config::Config;
use reqwest;
use serde::Deserialize;
use std::time::Duration;
use tokio::{task, time::sleep};

// Struct for configuration
#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    sonarr: ServiceConfig,
    radarr: ServiceConfig,
    jellyfin: ServiceConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct ServiceConfig {
    url: String,
    api_key: String,
}

#[tokio::main]
async fn main() {
    // Load configuration
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Failed to load configuration")
        .try_deserialize::<AppConfig>()
        .expect("Invalid configuration format");

    // Start monitoring tasks
    let sonarr_handle = task::spawn(queuery_sonarr(
        settings.sonarr.clone(),
        settings.jellyfin.clone(),
    ));
    let radarr_handle = task::spawn(queuery_radarr(
        settings.radarr.clone(),
        settings.jellyfin.clone(),
    ));

    let _ = tokio::join!(sonarr_handle, radarr_handle);
}

async fn queuery_sonarr(sonarr_config: ServiceConfig, jellyfin_config: ServiceConfig) {
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
                        // Update Jellyfin, to rescan Library
                        let _ = trigger_jellyfin_refresh(&jellyfin_config).await;
                    }
                }
                last_total_records = Some(response.totalRecords);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        sleep(Duration::from_millis(500)).await;
    }
}

async fn queuery_radarr(radarr_config: ServiceConfig, jellyfin_config: ServiceConfig) {
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
                        // Update Jellyfin, to rescan Library
                        let _ = trigger_jellyfin_refresh(&jellyfin_config).await;
                    }
                }
                last_total_records = Some(response.totalRecords);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        sleep(Duration::from_millis(500)).await;
    }
}

// Function to query API from Sonarr
async fn fetch_data_soanrr(url: &str) -> Result<ApiFieldsSonarr, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<ApiFieldsSonarr>().await?;
    Ok(response)
}

// Function to query API from Radarr
async fn fetch_data_radarr(url: &str) -> Result<ApiFieldsRadarr, reqwest::Error> {
    let response = reqwest::get(url).await?.json::<ApiFieldsRadarr>().await?;
    Ok(response)
}

// Structure for JSON-Response from Sonarr
#[derive(Debug, Deserialize)]
struct ApiFieldsSonarr {
    totalRecords: i32,
}

// Structure for JSON-Response from Radarr
#[derive(Debug, Deserialize)]
struct ApiFieldsRadarr {
    totalRecords: i32,
}

// Function to trigger Jellyfin library refresh
async fn trigger_jellyfin_refresh(config: &ServiceConfig) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/Library/Refresh", config.url);

    let response = client
        .post(&url)
        .header(
            "Authorization",
            format!("MediaBrowser Token=\"{}\"", config.api_key),
        )
        .send()
        .await?;

    if response.status().is_success() {
        println!("[Jellyfin] Library refresh triggered successfully");
    } else {
        eprintln!(
            "[Jellyfin] Failed to trigger library refresh: {}",
            response.status()
        );
    }
    Ok(())
}
