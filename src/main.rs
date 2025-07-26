mod chromecast;
mod config;
mod player_controls;
pub mod server;
mod settings;
mod utils;

use clap::{Parser, Subcommand};
use id3::{Tag, TagLike};
use playlist_decoder::decode_playlist;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    settings: settings::Settings,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Handles audio file operations
    Audio {
        /// Path to the audio file
        #[arg(short, long)]
        file: String,
    },
    /// Handles playlist operations
    Playlist {
        /// Path to the playlist file
        #[arg(short, long)]
        file: String,
    },
    /// Handles HTML file operations
    Html {
        /// Path to the HTML file
        #[arg(short, long)]
        file: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cli_settings = args.settings;

    let file_and_env_settings = config::get_configuration()?;
    let settings = utils::merge_settings(cli_settings, file_and_env_settings);

    if settings.show_options {
        println!("{settings:#?}");
        return Ok(());
    }

    if let Some(command) = args.command {
        match command {
            Commands::Audio { file } => {
                handle_audio_file(&file);
            }
            Commands::Playlist { file } => {
                handle_playlist_file(&file);
            }
            Commands::Html { file } => {
                handle_html_file(&file);
            }
        }
    } else if let Some(media_path) = &settings.media_path {
        let devices = chromecast::discover_devices()?;
        let device_info = chromecast::select_device(&settings, devices)?;

        let (device, transport_id, session_id) =
            if media_path.starts_with("http://") || media_path.starts_with("https://") {
                chromecast::cast(&device_info, settings.clone()).await?
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

                let (device, transport_id, session_id) =
                    chromecast::cast(&device_info, settings_with_url).await?;

                if settings.exit {
                    let _ = tx.send(());
                    server_handle.await?;
                }
                (device, transport_id, session_id)
            };
        player_controls::handle_player_controls(device, transport_id, session_id).await?;
    }

    Ok(())
}

fn handle_audio_file(file_path: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("Error: File not found at {file_path}");
        return;
    }

    match Tag::read_from_path(path) {
        Ok(tag) => {
            println!("--- Audio Metadata ---");
            if let Some(title) = tag.title() {
                println!("Title: {title}");
            }
            if let Some(artist) = tag.artist() {
                println!("Artist: {artist}");
            }
            if let Some(album) = tag.album() {
                println!("Album: {album}");
            }
            if let Some(year) = tag.year() {
                println!("Year: {year}");
            }
            if let Some(genre) = tag.genre() {
                println!("Genre: {genre}");
            }
            if let Some(track) = tag.track() {
                println!("Track: {track}");
            }
            if let Some(total_tracks) = tag.total_tracks() {
                println!("Total Tracks: {total_tracks}");
            }
            if let Some(disc) = tag.disc() {
                println!("Disc: {disc}");
            }
            if let Some(total_discs) = tag.total_discs() {
                println!("Total Discs: {total_discs}");
            }
            if let Some(comment) = tag.comments().next() {
                println!("Comment: {}", comment.text);
            }

            for picture in tag.pictures() {
                println!(
                    "Cover Art: MIME Type = {}, Size = {} bytes",
                    picture.mime_type,
                    picture.data.len()
                );
            }
        }
        Err(e) => {
            eprintln!("Error reading tags from {file_path}: {e}");
        }
    }
}

fn handle_playlist_file(file_path: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("Error: Playlist file not found at {file_path}");
        return;
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading playlist file {file_path}: {e}");
            return;
        }
    };

    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    match extension.to_lowercase().as_str() {
        "m3u" | "m3u8" | "pls" | "xspf" => {
            // playlist-decoder handles these
            let playlist_entries = decode_playlist(&content);
            println!("--- {} Playlist ---", extension.to_uppercase());
            for entry in playlist_entries {
                println!("Path: {entry}");
            }
        }
        "cue" => {
            handle_cue_file(&content);
        }
        _ => {
            eprintln!("Error: Unsupported playlist format for file {file_path}");
        }
    }
}

fn handle_cue_file(content: &str) {
    println!("--- CUE Sheet ---");
    let mut current_file = String::new();
    let mut current_track_number = 0;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("FILE") {
            if let Some(file_path_start) = line.find('"') {
                if let Some(file_path_end) = line[file_path_start + 1..].find('"') {
                    current_file =
                        line[file_path_start + 1..file_path_start + 1 + file_path_end].to_string();
                    println!("File: {current_file}");
                }
            }
        } else if line.starts_with("TRACK") {
            if let Some(track_num_str) = line.split_whitespace().nth(1) {
                if let Ok(track_num) = track_num_str.parse::<u32>() {
                    current_track_number = track_num;
                    println!("  Track: {current_track_number}");
                }
            }
        } else if line.starts_with("TITLE") {
            if let Some(title_start) = line.find('"') {
                if let Some(title_end) = line[title_start + 1..].find('"') {
                    let title = line[title_start + 1..title_start + 1 + title_end].to_string();
                    println!("    Title: {title}");
                }
            }
        } else if line.starts_with("INDEX") {
            if let Some(index_parts) = line.split_whitespace().nth(2) {
                println!("    Index: {index_parts}");
            }
        }
    }
}

fn handle_html_file(file_path: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("Error: HTML file not found at {file_path}");
        return;
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading HTML file {file_path}: {e}");
            return;
        }
    };

    let document = Html::parse_document(&content);
    let selector = Selector::parse(
        "a[href$='.mp3'], a[href$='.flac'], a[href$='.ogg'], a[href$='.wav'], a[href$='.m4a']",
    )
    .unwrap();

    println!("--- Audio Links in HTML ---");
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            println!("Link: {href}");
        }
    }
}
