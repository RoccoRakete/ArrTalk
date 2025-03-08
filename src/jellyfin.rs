// jellyfin.rs
use crate::ServiceConfig;
use reqwest;

pub async fn trigger_jellyfin_refresh(config: &ServiceConfig) -> Result<(), reqwest::Error> {
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
