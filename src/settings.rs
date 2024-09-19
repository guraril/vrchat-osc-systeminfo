use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    #[serde(default)]
    pub log_level: String,
    #[serde(default)]
    pub osc_port: OscPort,
    #[serde(default)]
    pub send_chatbox: SendChatbox,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            log_level: "warn".into(),
            osc_port: OscPort::default(),
            send_chatbox: SendChatbox::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OscPort {
    #[serde(default)]
    pub send_to_vrchat: u16,
    #[serde(default)]
    pub recv_from_vrchat: u16,
}

impl Default for OscPort {
    fn default() -> Self {
        OscPort {
            send_to_vrchat: 9000,
            recv_from_vrchat: 9001,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SendChatbox {
    #[serde(default)]
    pub presets: Vec<String>,
    #[serde(default)]
    pub preset_key: String,
}

impl Default for SendChatbox {
    fn default() -> Self {
        SendChatbox {
            presets: vec![String::from(
                "CPU Usage: ${cpu_usage}%\nGPU Usage: ${gpu_usage}%\nMem: ${used_mem}/${total_mem}",
            )],
            preset_key: String::from("/avatar/parameters/SysinfoPreset"),
        }
    }
}
