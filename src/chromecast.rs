use crate::settings::Settings;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use rust_cast::channels::media::{Media, StreamType};
use rust_cast::channels::receiver::CastDeviceApp;
use rust_cast::CastDevice;
use std::io::{self, Write};
use std::str::FromStr;
use std::time::Duration;

pub fn discover_devices() -> anyhow::Result<Vec<ServiceInfo>> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse("_googlecast._tcp.local.")?;

    let mut chromecasts = Vec::new();
    let timeout = Duration::from_secs(5);
    let start_time = std::time::Instant::now();

    println!("Searching for Chromecast devices...");
    while std::time::Instant::now() - start_time < timeout {
        if let Ok(event) = receiver.recv_timeout(timeout - (std::time::Instant::now() - start_time))
        {
            if let ServiceEvent::ServiceResolved(info) = event {
                println!(
                    "Found Chromecast: {} at {}:{}",
                    info.get_fullname(),
                    info.get_addresses().iter().next().unwrap(),
                    info.get_port()
                );
                chromecasts.push(info);
            }
        } else {
            break;
        }
    }

    Ok(chromecasts)
}

pub fn select_device(
    settings: &Settings,
    devices: Vec<ServiceInfo>,
) -> anyhow::Result<ServiceInfo> {
    if let Some(address) = &settings.address {
        // Find device by address
        for device in devices {
            if device
                .get_addresses()
                .iter()
                .any(|a| a.to_string() == *address)
            {
                return Ok(device);
            }
        }
        return Err(anyhow::anyhow!("Device with address {} not found", address));
    }

    if let Some(device_name) = &settings.device {
        // Find device by name
        for device in devices {
            if device.get_fullname().starts_with(device_name) {
                return Ok(device);
            }
        }
        return Err(anyhow::anyhow!(
            "Device with name {} not found",
            device_name
        ));
    }

    if devices.len() == 1 {
        return Ok(devices.into_iter().next().unwrap());
    }

    if devices.is_empty() {
        return Err(anyhow::anyhow!("No Chromecast devices found"));
    }

    // Prompt user to select a device
    println!("Multiple Chromecast devices found:");
    for (i, device) in devices.iter().enumerate() {
        println!("{}: {}", i + 1, device.get_fullname());
    }

    loop {
        print!("Enter the number of the device to use: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if let Ok(index) = input.parse::<usize>() {
            if index > 0 && index <= devices.len() {
                return Ok(devices.into_iter().nth(index - 1).unwrap());
            }
        }

        println!("Invalid number. Please try again.");
    }
}

pub async fn cast(device_info: &ServiceInfo, settings: Settings) -> anyhow::Result<(CastDevice, String, String)> {
    let ip = device_info
        .get_addresses()
        .iter()
        .next()
        .unwrap()
        .to_string();
    let port = device_info.get_port();

    let device = CastDevice::connect_without_host_verification(ip.to_owned(), port)?;
    let default_media_receiver_app = CastDeviceApp::from_str("CC1AD845").unwrap();

    let app = device.receiver.launch_app(&default_media_receiver_app)?;

    let media = Media {
        content_id: settings.media_path.as_ref().unwrap().to_string(),
        content_type: settings
            .media_type
            .clone()
            .unwrap_or_else(|| "video/mp4".to_string()),
        stream_type: StreamType::Buffered,
        duration: None,
        metadata: None,
    };

    device
        .media
        .load(app.transport_id.as_str(), app.session_id.as_str(), &media)?;

    Ok((device, app.transport_id, app.session_id))
}
