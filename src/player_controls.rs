use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use rust_cast::CastDevice;
use tokio::sync::mpsc;

pub async fn handle_player_controls(
    device: CastDevice<'_>,
    transport_id: String,
    _session_id: String,
) -> Result<(), anyhow::Error> {
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
                // Play/Pause toggle
                println!("Toggling play/pause...");
                if let Ok(status) = device.media.get_status(&transport_id, None).await {
                    if let Some(media_status) = status.entries.first() {
                        if media_status.player_state
                            == rust_cast::channels::media::PlayerState::Playing
                        {
                            let _ = device
                                .media
                                .pause(&transport_id, media_status.media_session_id)
                                .await;
                        } else {
                            let _ = device
                                .media
                                .play(&transport_id, media_status.media_session_id)
                                .await;
                        }
                    }
                }
            }
            KeyCode::Char('m') => {
                // Mute toggle
                println!("Toggling mute...");
                if let Ok(receiver_status) = device.receiver.get_status().await {
                    let current_volume = &receiver_status.volume;
                    let _ = device
                        .receiver
                        .set_volume(rust_cast::channels::receiver::Volume {
                            level: current_volume.level,
                            muted: Some(!current_volume.muted.unwrap_or(false)), // unwrap_or(false) を追加
                        })
                        .await;
                }
                }
            }
            // KeyCode::Char('t') => {
            //     // Subtitle toggle (temporarily disabled due to rust-cast API changes)
            //     println!("Subtitle toggle is temporarily disabled.");
            // }
            KeyCode::Up => {
                // Volume up
                println!("Volume up...");
                if let Ok(receiver_status) = device.receiver.get_status().await {
                    let current_volume = &receiver_status.volume;
                    let new_level = (current_volume.level.unwrap_or(0.0) + 0.05).min(1.0);
                    let _ = device
                        .receiver
                        .set_volume(rust_cast::channels::receiver::Volume {
                            level: Some(new_level),
                            muted: current_volume.muted,
                        })
                        .await;
                }
                }
            }
            KeyCode::Down => {
                // Volume down
                println!("Down...");
                if let Ok(receiver_status) = device.receiver.get_status().await {
                    let current_volume = &receiver_status.volume;
                    let new_level = (current_volume.level.unwrap_or(0.0) - 0.05).max(0.0);
                    let _ = device
                        .receiver
                        .set_volume(rust_cast::channels::receiver::Volume {
                            level: Some(new_level),
                            muted: current_volume.muted,
                        })
                        .await;
                }
                }
            }
            KeyCode::Left => {
                // Seek backward
                println!("Seeking backward...");
                if let Ok(status) = device.media.get_status(&transport_id, None).await {
                    if let Some(media_status) = status.entries.first() {
                        let current_time = media_status.current_time.unwrap_or(0.0);
                        let new_time = (current_time - 10.0).max(0.0); // Seek back 10 seconds
                        let _ = device.media.seek(
                            &transport_id,
                            media_status.media_session_id,
                            Some(new_time),
                            None,
                        )
                        .await;
                    }
                }
            }
            KeyCode::Right => {
                // Seek forward
                println!("Seeking forward...");
                if let Ok(status) = device.media.get_status(&transport_id, None).await {
                    if let Some(media_status) = status.entries.first() {
                        let current_time = media_status.current_time.unwrap_or(0.0);
                        let media_duration = media_status
                            .media
                            .as_ref()
                            .and_then(|m| m.duration)
                            .unwrap_or(current_time); // Use current_time if duration is not available
                        let new_time = (current_time + 10.0).min(media_duration); // Cap at media_duration
                        let _ = device.media.seek(
                            &transport_id,
                            media_status.media_session_id,
                            Some(new_time),
                            None,
                        )
                        .await;
                    }
                }
            }
            // KeyCode::Char('p') => {
            //     // Previous item in playlist (temporarily disabled due to rust-cast API changes)
            //     println!("Previous item in playlist is temporarily disabled.");
            // }
            // KeyCode::Char('n') => {
            //     // Next item in playlist (temporarily disabled due to rust-cast API changes)
            //     println!("Next item in playlist is temporarily disabled.");
            // }
            KeyCode::Char('s') => {
                // Stop playback
                println!("Stopping playback...");
                if let Ok(status) = device.media.get_status(&transport_id, None).await {
                    if let Some(media_status) = status.entries.first() {
                        let _ = device
                            .media
                            .stop(&transport_id, media_status.media_session_id)
                            .await;
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
