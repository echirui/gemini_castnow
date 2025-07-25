use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use rust_cast::CastDevice;
use tokio::sync::mpsc;

pub async fn handle_player_controls(mut device: CastDevice<'_>, transport_id: String, session_id: String) -> Result<(), anyhow::Error> {
    enable_raw_mode()?;
    let mut reader = event::EventStream::new();
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            let event = reader.next().await;
            if let Some(Ok(Event::Key(key_event))) = event {
                if key_event.kind == KeyEventKind::Press {
                    tx.send(key_event.code).unwrap();
                }
            }
        }
    });

    while let Some(key_code) = rx.recv().await {
        match key_code {
            KeyCode::Char(' ') => {
                // Play/Pause
                println!("Toggling play/pause...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let current_volume = &media_status.volume;
                        let _ = device.receiver.set_volume(rust_cast::channels::receiver::Volume { level: current_volume.level, muted: !current_volume.muted });
                    }
                }
            }
            KeyCode::Char('t') => {
                // Subtitle toggle
                println!("Toggling subtitles (not yet implemented)");
            }
            KeyCode::Up => {
                // Volume up
                println!("Volume up...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let current_volume = &media_status.volume;
                        let new_level = (current_volume.level + 0.05).min(1.0);
                        let _ = device.receiver.set_volume(rust_cast::channels::receiver::Volume { level: new_level, muted: current_volume.muted });
                    }
                }
            }
            KeyCode::Down => {
                // Volume down
                println!("Down...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let current_volume = &media_status.volume;
                        let new_level = (current_volume.level - 0.05).max(0.0);
                        let _ = device.receiver.set_volume(rust_cast::channels::receiver::Volume { level: new_level, muted: current_volume.muted });
                    }
                }
            }
            KeyCode::Left => {
                // Seek backward
                println!("Seeking backward...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let current_time = media_status.current_time.unwrap_or(0.0);
                        let new_time = (current_time - 10.0).max(0.0); // Seek back 10 seconds
                        let _ = device.media.seek(&transport_id, media_status.media_session_id, Some(new_time), None);
                    }
                }
            }
            KeyCode::Right => {
                // Seek forward
                println!("Seeking forward...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let current_time = media_status.current_time.unwrap_or(0.0);
                        let new_time = current_time + 10.0; // Seek forward 10 seconds
                        let _ = device.media.seek(&transport_id, media_status.media_session_id, Some(new_time), None);
                    }
                }
            }
            KeyCode::Char('p') => {
                // Previous item in playlist
                println!("Previous item in playlist (not yet implemented)");
            }
            KeyCode::Char('n') => {
                // Next item in playlist
                println!("Next item in playlist (not yet implemented)");
            }
            KeyCode::Char('s') => {
                // Stop playback
                println!("Stopping playback...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let _ = device.media.stop(&transport_id, media_status.media_session_id);
                    }
                }
            }
            KeyCode::Char('q') => {
                // Quit
                println!("Quit (q)");
                break;
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    Ok(())
}
