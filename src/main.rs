use config::Config;
use serde::Deserialize;
use tokio::task;

mod jellyfin;
mod radarr;
mod sonarr;

// Struct Config
#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    sonarr: ServiceConfig,
    radarr: ServiceConfig,
    jellyfin: ServiceConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct ServiceConfig {
    enable: bool,
    url: String,
    api_key: String,
}

#[tokio::main]
async fn main() {
    // Load Config
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Failed to load configuration")
        .try_deserialize::<AppConfig>()
        .expect("Invalid configuration format");

    // Start Task to watch API's if enabled
    let mut handles = vec![];

    if settings.sonarr.enable {
        let sonarr_handle = task::spawn(sonarr::queuery_sonarr(
            settings.sonarr.clone(),
            settings.jellyfin.clone(),
        ));
        handles.push(sonarr_handle);
    } else {
        println!("[WARNING!] Watchdog for Sonarr is disabled inside the config.toml!")
    }

    if settings.radarr.enable {
        let radarr_handle = task::spawn(radarr::queuery_radarr(
            settings.radarr.clone(),
            settings.jellyfin.clone(),
        ));
        handles.push(radarr_handle);
    } else {
        println!("[WARNING!] Watchdog for Radarr is disabled inside the config.toml!")
    }

    // Await all spawned tasks
    for handle in handles {
        let _ = handle.await;
    }
}
