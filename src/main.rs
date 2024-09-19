mod settings;
mod stat;

use log::{debug, error, info, warn};
use rosc::{decoder, encoder, OscMessage, OscPacket, OscType};
use settings::Settings;
use stat::Statistics;
use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use tokio::{net::UdpSocket, sync::mpsc, time::sleep};

#[tokio::main]
async fn main() {
    let settings = match fs::read_to_string("Settings/settings.json") {
        Ok(json) => serde_json::from_str(json.as_str()).unwrap_or(Settings::default()),
        Err(_) => Settings::default(),
    };
    init_logger(settings.log_level);
    let addr_recv_from_vrchat = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        settings.osc_port.recv_from_vrchat,
    );
    let addr_send_to_vrchat = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        settings.osc_port.send_to_vrchat,
    );

    let (tx1, mut rx1) = mpsc::channel(2);
    let recv_handle = tokio::spawn(async move {
        let Ok(sock) = UdpSocket::bind(addr_recv_from_vrchat).await else {
            error!("Cannot initialize the receiver. Now you cannot change preset.");
            return;
        };
        loop {
            let mut buf = [0; 1024 * 2]; // 2KiB
            let Ok((_len, _src)) = sock.recv_from(&mut buf).await else {
                error!(
                    "Network issue has been detected. Please ensure your network connection is good."
                );
                continue;
            };
            if let Ok((_, msg)) = decoder::decode_udp(&buf) {
                match msg {
                    OscPacket::Message(message) => {
                        if message.addr == settings.send_chatbox.preset_key {
                            tx1.send(message.args[0].clone().int().unwrap_or(0) as usize)
                                .await
                                .expect("Failed to send data to another thread.");
                        };
                    }
                    OscPacket::Bundle(_) => {
                        warn!("OscBundle is not supported.");
                    }
                };
            };
        }
    });
    let send_handle = tokio::spawn(async move {
        let sock = UdpSocket::bind("127.0.0.1:0")
            .await
            .expect("Cannot bind udp socket.");
        let mut preset_number = 0;
        loop {
            while let Ok(preset) = rx1.try_recv() {
                debug!("preset updated.");
                preset_number = preset;
            }
            if preset_number >= settings.send_chatbox.presets.len() {
                preset_number = settings.send_chatbox.presets.len() - 1;
            };
            let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
                addr: "/chatbox/input".to_string(),
                args: vec![
                    OscType::String(format_string(String::from(
                        settings.send_chatbox.presets[preset_number].as_str(),
                    ))),
                    OscType::Bool(true),
                    OscType::Bool(false),
                ],
            }))
            .expect("Failed to encode message.");
            sock.send_to(&msg_buf, addr_send_to_vrchat)
                .await
                .unwrap_or_else(|e| {
                    debug!("{}", e);
                    error!("Failed to send data.");
                    0
                });
            sleep(Duration::from_secs(1)).await;
        }
    });
    let (_h1, _h2) = (send_handle.await.ok(), recv_handle.await.ok());
}

fn format_string(data: String) -> String {
    let mut statistics = Statistics::new();
    data.replace("${os_name}", &statistics.get_os_name())
        .replace("${os_version}", &statistics.get_os_version())
        .replace("${cpu_arch}", &statistics.get_cpu_arch())
        .replace("${cpu_brand}", &statistics.get_cpu_brand())
        .replace("${gpu_brand}", &statistics.get_gpu_brand())
        .replace(
            "${cpu_usage}",
            &statistics.get_average_cpu_usage().to_string(),
        )
        .replace(
            "${gpu_usage}",
            &statistics.get_average_gpu_usage().to_string(),
        )
        .replace(
            "${mem_usage}",
            &((statistics.get_used_mem() / statistics.get_total_mem() * 100f32 * 100f32).round()
                / 100f32)
                .to_string(),
        )
        .replace(
            "${vram_usage}",
            &((statistics.get_used_vram() / statistics.get_total_vram() * 100f32 * 100f32).round()
                / 100f32)
                .to_string(),
        )
        .replace(
            "${cpu_temp}",
            &statistics.get_average_cpu_temp().to_string(),
        )
        .replace("${gpu_temp", &statistics.get_average_gpu_temp().to_string())
        .replace("${total_mem}", &statistics.get_total_mem().to_string())
        .replace("${used_mem}", &statistics.get_used_mem().to_string())
        .replace("${total_vram}", &statistics.get_total_vram().to_string())
        .replace("${used_vram}", &statistics.get_used_vram().to_string())
}

fn init_logger(level: String) {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "none" | "off" => log::LevelFilter::Off,
        "debug" => log::LevelFilter::Debug,
        "info" | "information" => log::LevelFilter::Info,
        "warn" | "warning" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Warn,
    };
    if fern::Dispatch::new()
        .level(log_level)
        .format(|out, message, record| {
            out.finish(format_args!("[{}]: {}", record.level(), message))
        })
        .chain(std::io::stdout())
        .apply()
        .is_ok()
    {
        info!("Logger initialized. Log level: {}", log_level);
    } else {
        warn!("Failed to initialize logger");
    }
}
