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
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        if media_status.player_state
                            == rust_cast::channels::media::PlayerState::Playing
                        {
                            let _ = device
                                .media
                                .pause(&transport_id, media_status.media_session_id);
                        } else {
                            let _ = device
                                .media
                                .play(&transport_id, media_status.media_session_id);
                        }
                    }
                }
            }
            KeyCode::Char('m') => {
                // Mute toggle
                println!("Toggling mute...");
                if let Ok(receiver_status) = device.receiver.get_status() {
                    let current_volume = &receiver_status.volume;
                    let _ = device
                        .receiver
                        .set_volume(rust_cast::channels::receiver::Volume {
                            level: current_volume.level,
                            muted: Some(!current_volume.muted.unwrap_or(false)), // unwrap_or(false) を追加
                        });
                }
            }
            KeyCode::Char('t') => {
                // Subtitle toggle
                println!("Toggling subtitles...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let mut active_track_ids = media_status.active_track_ids.clone().unwrap_or_default();
                        let text_track_id = media_status.tracks.as_ref().and_then(|tracks| {
                            tracks.iter().find(|track| track.track_type == rust_cast::channels::media::TrackType::Text)
                                .map(|track| track.track_id)
                        });

                        if let Some(track_id) = text_track_id {
                            if active_track_ids.contains(&track_id) {
                                // Subtitle is active, deactivate it
                                active_track_ids.retain(|&id| id != track_id);
                                println!("Subtitles off");
                            } else {
                                // Subtitle is inactive, activate it
                                active_track_ids.push(track_id);
                                println!("Subtitles on");
                            }
                            let _ = device.media.set_active_media_tracks(
                                &transport_id,
                                media_status.media_session_id,
                                &active_track_ids,
                            );
                        } else {
                            println!("No text tracks found.");
                        }
                    }
                }
            }
            KeyCode::Up => {
                // Volume up
                println!("Volume up...");
                if let Ok(receiver_status) = device.receiver.get_status() {
                    let current_volume = &receiver_status.volume;
                    let new_level = (current_volume.level.unwrap_or(0.0) + 0.05).min(1.0);
                    let _ = device
                        .receiver
                        .set_volume(rust_cast::channels::receiver::Volume {
                            level: Some(new_level),
                            muted: current_volume.muted,
                        });
                }
            }
            KeyCode::Down => {
                // Volume down
                println!("Down...");
                if let Ok(receiver_status) = device.receiver.get_status() {
                    let current_volume = &receiver_status.volume;
                    let new_level = (current_volume.level.unwrap_or(0.0) - 0.05).max(0.0);
                    let _ = device
                        .receiver
                        .set_volume(rust_cast::channels::receiver::Volume {
                            level: Some(new_level),
                            muted: current_volume.muted,
                        });
                }
            }
            KeyCode::Left => {
                // Seek backward
                println!("Seeking backward...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let current_time = media_status.current_time.unwrap_or(0.0);
                        let new_time = (current_time - 10.0).max(0.0); // Seek back 10 seconds
                        let _ = device.media.seek(
                            &transport_id,
                            media_status.media_session_id,
                            Some(new_time),
                            None,
                        );
                    }
                }
            }
            KeyCode::Right => {
                // Seek forward
                println!("Seeking forward...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
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
                        );
                    }
                }
            }
            KeyCode::Char('p') => {
                // Previous item in playlist
                println!("Previous item in playlist...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        if let Some(current_item_id) = media_status.current_item_id {
                            if let Some(items) = &media_status.items {
                                if let Some(current_index) = items.iter().position(|item| item.item_id == current_item_id) {
                                    if current_index > 0 {
                                        let previous_item_id = items[current_index - 1].item_id;
                                        let _ = device.media.queue_update(
                                            &transport_id,
                                            media_status.media_session_id,
                                            Some(previous_item_id),
                                            None,
                                            None,
                                            None,
                                        );
                                    } else {
                                        println!("Already at the beginning of the playlist.");
                                    }
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Char('n') => {
                // Next item in playlist
                println!("Next item in playlist...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        if let Some(current_item_id) = media_status.current_item_id {
                            if let Some(items) = &media_status.items {
                                if let Some(current_index) = items.iter().position(|item| item.item_id == current_item_id) {
                                    if current_index < items.len() - 1 {
                                        let next_item_id = items[current_index + 1].item_id;
                                        let _ = device.media.queue_update(
                                            &transport_id,
                                            media_status.media_session_id,
                                            Some(next_item_id),
                                            None,
                                            None,
                                            None,
                                        );
                                    } else {
                                        println!("Already at the end of the playlist.");
                                    }
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Char('s') => {
                // Stop playback
                println!("Stopping playback...");
                if let Ok(status) = device.media.get_status(&transport_id, None) {
                    if let Some(media_status) = status.entries.first() {
                        let _ = device
                            .media
                            .stop(&transport_id, media_status.media_session_id);
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
