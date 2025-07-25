use clap::Parser;
use gemini_castnow::{Cli, Commands, start_server};
use std::path::PathBuf;
use walkdir::WalkDir;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;
use tokio::time::sleep;
use rust_cast::CastDevice;
use rust_cast::channels::media::{Media, StreamType};
use rust_cast::channels::receiver::CastDeviceApp;
use std::str::FromStr;
use std::io::{self, Write};
use librqbit::{Session, AddTorrent};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::PlayFile { file } => {
            println!("Playing local file: {}", file);
            let file_path = PathBuf::from(file);
            if !file_path.exists() {
                eprintln!("Error: File not found: {}", file);
                return;
            }

            let (tx, rx) = tokio::sync::oneshot::channel();
            let (server_socket_addr, server_handle) = match start_server(file_path, rx).await {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error starting server: {}", e);
                    return;
                }
            };

            println!("Searching for Chromecast devices...");
            let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
            let receiver = mdns.browse("_googlecast._tcp.local.").expect("Failed to browse mDNS services");

            let mut chromecasts = Vec::new();
            let timeout = Duration::from_secs(5);
            let start_time = tokio::time::Instant::now();

            while tokio::time::Instant::now() - start_time < timeout {
                match tokio::time::timeout(timeout - (tokio::time::Instant::now() - start_time), receiver.recv_async()).await {
                    Ok(Ok(event)) => {
                        if let ServiceEvent::ServiceResolved(info) = event {
                            println!("Found Chromecast: {} at {}:{}", info.get_fullname(), info.get_addresses().iter().next().unwrap(), info.get_port());
                            chromecasts.push(info);
                        }
                    },
                    Ok(Err(e)) => eprintln!("mDNS receive error: {}", e),
                    Err(_) => break, // Timeout
                }
            }

            if chromecasts.is_empty() {
                eprintln!("No Chromecast devices found.");
                // Send shutdown signal to server before exiting
                let _ = tx.send(());
                return;
            }

            // For now, just pick the first one
            let chromecast_info = &chromecasts[0];
            let chromecast_ip = chromecast_info.get_addresses().iter().next().unwrap().to_string();
            let chromecast_port = chromecast_info.get_port();
            let chromecast_name = chromecast_info.get_fullname();

            println!("Attempting to cast to {}: {}:{}", chromecast_name, chromecast_ip, chromecast_port);

            let mut device = match CastDevice::connect_without_host_verification(chromecast_ip.as_str(), chromecast_port) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to connect to Chromecast: {}", e);
                    // Send shutdown signal to server before exiting
                    let _ = tx.send(());
                    return;
                }
            };

            let media_url = format!("http://{}/", server_socket_addr);
            println!("Casting URL: {}", media_url);

            // Corrected: Use CastDeviceApp::from_str
            let default_media_receiver_app = CastDeviceApp::from_str("CC1AD845").unwrap();

            match device.receiver.launch_app(&default_media_receiver_app) { 
                Ok(app) => {
                    println!("Launched app: {}", app.app_id);
                    let media = Media { content_id: media_url.to_string(), content_type: "video/mp4".to_string(), stream_type: StreamType::Buffered, duration: None, metadata: None };
                    match device.media.load(app.transport_id.as_str(), app.session_id.as_str(), &media) { 
                        Ok(_) => println!("Media loaded successfully!"),
                        Err(e) => eprintln!("Failed to load media: {}", e),
                    }
                },
                Err(e) => eprintln!("Failed to launch app: {}", e),
            }

            // Send shutdown signal to server after casting
            let _ = tx.send(());
            let _ = server_handle.await; // Await the server task to ensure it shuts down
        }
        Commands::PlayDir { dir } => {
            println!("Playing media from directory: {}", dir);
            let dir_path = PathBuf::from(dir);
            if !dir_path.is_dir() {
                eprintln!("Error: Directory not found or is not a directory: {}", dir);
                return;
            }

            let mut media_files = Vec::new();
            for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path().to_path_buf();
                    // Basic media file type check (can be expanded)
                    if let Some(ext) = path.extension() {
                        if ext == "mp4" || ext == "mkv" || ext == "avi" || ext == "mp3" {
                            media_files.push(path);
                        }
                    }
                }
            }

            if media_files.is_empty() {
                println!("No supported media files found in directory: {}", dir);
                return;
            }

            println!("Found media files:");
            for (i, file) in media_files.iter().enumerate() {
                println!("{}: {}", i + 1, file.display());
            }

            let selected_file_path: PathBuf;
            loop {
                print!("Enter the number of the file to play: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if let Ok(index) = input.parse::<usize>() {
                    if index > 0 && index <= media_files.len() {
                        selected_file_path = media_files[index - 1].clone();
                        break;
                    } else {
                        println!("Invalid number. Please try again.");
                    }
                } else {
                    println!("Invalid input. Please enter a number.");
                }
            }

            println!("Playing selected file: {}", selected_file_path.display());

            // Reuse PlayFile logic
            let (tx, rx) = tokio::sync::oneshot::channel();
            let (server_socket_addr, server_handle) = match start_server(selected_file_path, rx).await {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error starting server: {}", e);
                    return;
                }
            };

            println!("Searching for Chromecast devices...");
            let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
            let receiver = mdns.browse("_googlecast._tcp.local.").expect("Failed to browse mDNS services");

            let mut chromecasts = Vec::new();
            let timeout = Duration::from_secs(5);
            let start_time = tokio::time::Instant::now();

            while tokio::time::Instant::now() - start_time < timeout {
                match tokio::time::timeout(timeout - (tokio::time::Instant::now() - start_time), receiver.recv_async()).await {
                    Ok(Ok(event)) => {
                        if let ServiceEvent::ServiceResolved(info) = event {
                            println!("Found Chromecast: {} at {}:{}", info.get_fullname(), info.get_addresses().iter().next().unwrap(), info.get_port());
                            chromecasts.push(info);
                        }
                    },
                    Ok(Err(e)) => eprintln!("mDNS receive error: {}", e),
                    Err(_) => break, // Timeout
                }
            }

            if chromecasts.is_empty() {
                eprintln!("No Chromecast devices found.");
                let _ = tx.send(());
                return;
            }

            let chromecast_info = &chromecasts[0];
            let chromecast_ip = chromecast_info.get_addresses().iter().next().unwrap().to_string();
            let chromecast_port = chromecast_info.get_port();
            let chromecast_name = chromecast_info.get_fullname();

            println!("Attempting to cast to {}: {}:{}", chromecast_name, chromecast_ip, chromecast_port);

            let mut device = match CastDevice::connect_without_host_verification(chromecast_ip.as_str(), chromecast_port) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to connect to Chromecast: {}", e);
                    let _ = tx.send(());
                    return;
                }
            };

            let media_url = format!("http://{}/", server_socket_addr);
            println!("Casting URL: {}", media_url);

            let default_media_receiver_app = CastDeviceApp::from_str("CC1AD845").unwrap();

            match device.receiver.launch_app(&default_media_receiver_app) { 
                Ok(app) => {
                    println!("Launched app: {}", app.app_id);
                    let media = Media { content_id: media_url.to_string(), content_type: "video/mp4".to_string(), stream_type: StreamType::Buffered, duration: None, metadata: None };
                    match device.media.load(app.transport_id.as_str(), app.session_id.as_str(), &media) { 
                        Ok(_) => println!("Media loaded successfully!"),
                        Err(e) => eprintln!("Failed to load media: {}", e),
                    }
                },
                Err(e) => eprintln!("Failed to launch app: {}", e),
            }

            // Send shutdown signal to server after casting
            let _ = tx.send(());
            let _ = server_handle.await; // Await the server task to ensure it shuts down
        }
        Commands::PlayMultiple { files } => {
            println!("Playing multiple files: {:?}", files);
            // Discover Chromecasts once
            println!("Searching for Chromecast devices...");
            let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
            let receiver = mdns.browse("_googlecast._tcp.local.").expect("Failed to browse mDNS services");

            let mut chromecasts = Vec::new();
            let timeout = Duration::from_secs(5);
            let start_time = tokio::time::Instant::now();

            while tokio::time::Instant::now() - start_time < timeout {
                match tokio::time::timeout(timeout - (tokio::time::Instant::now() - start_time), receiver.recv_async()).await {
                    Ok(Ok(event)) => {
                        if let ServiceEvent::ServiceResolved(info) = event {
                            println!("Found Chromecast: {} at {}:{}", info.get_fullname(), info.get_addresses().iter().next().unwrap(), info.get_port());
                            chromecasts.push(info);
                        }
                    },
                    Ok(Err(e)) => eprintln!("mDNS receive error: {}", e),
                    Err(_) => break, // Timeout
                }
            }

            if chromecasts.is_empty() {
                eprintln!("No Chromecast devices found.");
                return;
            }

            let chromecast_info = &chromecasts[0];
            let chromecast_ip = chromecast_info.get_addresses().iter().next().unwrap().to_string();
            let chromecast_port = chromecast_info.get_port();
            let chromecast_name = chromecast_info.get_fullname();

            println!("Attempting to cast to {}: {}:{}", chromecast_name, chromecast_ip, chromecast_port);

            let mut device = match CastDevice::connect_without_host_verification(chromecast_ip.as_str(), chromecast_port) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to connect to Chromecast: {}", e);
                    return;
                }
            };

            let default_media_receiver_app = CastDeviceApp::from_str("CC1AD845").unwrap();

            for file in files {
                println!("Playing file: {}", file);
                let file_path = PathBuf::from(file);
                if !file_path.exists() {
                    eprintln!("Error: File not found: {}", file);
                    continue; // Skip to the next file
                }

                let (tx, rx) = tokio::sync::oneshot::channel();
                let (server_socket_addr, server_handle) = match start_server(file_path, rx).await {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error starting server for {}: {}", file, e);
                        continue; // Skip to the next file
                    }
                };

                let media_url = format!("http://{}/", server_socket_addr);
                println!("Casting URL: {}", media_url);

                match device.receiver.launch_app(&default_media_receiver_app) { 
                    Ok(app) => {
                        println!("Launched app: {}", app.app_id);
                        let media = Media { content_id: media_url.to_string(), content_type: "video/mp4".to_string(), stream_type: StreamType::Buffered, duration: None, metadata: None };
                        match device.media.load(app.transport_id.as_str(), app.session_id.as_str(), &media) { 
                            Ok(_) => println!("Media loaded successfully!"),
                            Err(e) => eprintln!("Failed to load media: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to launch app: {}", e),
                }

                let _ = tx.send(());
                let _ = server_handle.await;

                // Small delay between files (optional)
                sleep(Duration::from_secs(2)).await;
            }
        }
        Commands::PlayUrl { url } => {
            println!("Playing URL: {}", url);
            // Discover Chromecasts once
            println!("Searching for Chromecast devices...");
            let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");
            let receiver = mdns.browse("_googlecast._tcp.local.").expect("Failed to browse mDNS services");

            let mut chromecasts = Vec::new();
            let timeout = Duration::from_secs(5);
            let start_time = tokio::time::Instant::now();

            while tokio::time::Instant::now() - start_time < timeout {
                match tokio::time::timeout(timeout - (tokio::time::Instant::now() - start_time), receiver.recv_async()).await {
                    Ok(Ok(event)) => {
                        if let ServiceEvent::ServiceResolved(info) = event {
                            println!("Found Chromecast: {} at {}:{}", info.get_fullname(), info.get_addresses().iter().next().unwrap(), info.get_port());
                            chromecasts.push(info);
                        }
                    },
                    Ok(Err(e)) => eprintln!("mDNS receive error: {}", e),
                    Err(_) => break, // Timeout
                }
            }

            if chromecasts.is_empty() {
                eprintln!("No Chromecast devices found.");
                return;
            }

            let chromecast_info = &chromecasts[0];
            let chromecast_ip = chromecast_info.get_addresses().iter().next().unwrap().to_string();
            let chromecast_port = chromecast_info.get_port();
            let chromecast_name = chromecast_info.get_fullname();

            println!("Attempting to cast to {}: {}:{}", chromecast_name, chromecast_ip, chromecast_port);

            let mut device = match CastDevice::connect_without_host_verification(chromecast_ip.as_str(), chromecast_port) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to connect to Chromecast: {}", e);
                    return;
                }
            };

            let default_media_receiver_app = CastDeviceApp::from_str("CC1AD845").unwrap();

            match device.receiver.launch_app(&default_media_receiver_app) { 
                Ok(app) => {
                    println!("Launched app: {}", app.app_id);
                    let media = Media { content_id: url.to_string(), content_type: "video/mp4".to_string(), stream_type: StreamType::Buffered, duration: None, metadata: None };
                    match device.media.load(app.transport_id.as_str(), app.session_id.as_str(), &media) { 
                        Ok(_) => println!("Media loaded successfully!"),
                        Err(e) => eprintln!("Failed to load media: {}", e),
                    }
                },
                Err(e) => eprintln!("Failed to launch app: {}", e),
            }
        }
        Commands::PlayTorrent { torrent: _ } => {
            // TODO: Implement torrent playback using librqbit
            // The previous attempt to use librqbit encountered persistent compilation errors
            // related to accessing file information within the torrent metadata.
            // This feature will be implemented at a later stage.
            eprintln!("Torrent playback is not yet implemented.");
        }
    }
}