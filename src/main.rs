mod chromecast;
mod config;
mod server;
mod settings;
mod utils;

use clap::Parser;
use settings::Settings;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_settings = Settings::parse();
    let file_and_env_settings = config::get_configuration()?;

    let settings = utils::merge_settings(cli_settings, file_and_env_settings);

    if settings.show_options {
        println!("{settings:#?}");
        return Ok(());
    }

    if let Some(media_path) = &settings.media_path {
        let devices = chromecast::discover_devices()?;
        let device_info = chromecast::select_device(&settings, devices)?;

        if media_path.starts_with("http://") || media_path.starts_with("https://") {
            chromecast::cast(&device_info, &settings)?;
        } else {
            let file_path = PathBuf::from(media_path);
            if !file_path.exists() {
                eprintln!("Error: File not found: {media_path}");
                return Ok(());
            }

            let (tx, rx) = tokio::sync::oneshot::channel();
            let (server_addr, server_handle) = server::start_server(file_path, rx).await?;
            let media_url = format!("http://{server_addr}");
            let mut settings_with_url = settings.clone();
            settings_with_url.media_path = Some(media_url);

            chromecast::cast(&device_info, &settings_with_url)?;

            if settings.exit {
                let _ = tx.send(());
                server_handle.await?;
            }
        }
    }

    Ok(())
}
