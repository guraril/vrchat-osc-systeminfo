use log::warn;
use std::process::{Command, Stdio};
use sysinfo::{self, System};

pub struct Statistics {
    sys: System,
    os_name: String,
    os_version: String,
}
impl Statistics {
    pub fn new() -> Self {
        Statistics {
            sys: System::new_all(),
            os_name: System::name().unwrap_or("Unknown OS Name".into()),
            os_version: System::os_version().unwrap_or("Unknown OS Version".into()),
        }
    }

    pub fn get_os_name(&self) -> String {
        self.os_name.clone()
    }

    pub fn get_os_version(&self) -> String {
        self.os_version.clone()
    }

    pub fn get_cpu_arch(&self) -> String {
        System::cpu_arch().unwrap_or_else(|| {
            warn!("Failed to get CPU Architecture");
            "Unknown Architecture".into()
        })
    }

    pub fn get_cpu_brand(&self) -> String {
        // CPUが1つだけの環境を想定している。CPUを2つ以上を搭載できるマザーボードをお持ちの逸般の誤家庭の方はお引取りください。
        if let Some(first_cpu) = self.sys.cpus().first() {
            first_cpu.brand().into()
        } else {
            warn!("Failed to get CPU brand.");
            "Unknown CPU".into()
        }
    }

    #[cfg(target_os = "linux")]
    pub fn get_gpu_brand(&self) -> String {
        use std::process::Output;

        let output = Command::new("lspci")
            .stdout(Stdio::piped())
            .output()
            .unwrap_or(Output {
                status: Default::default(),
                stdout: Default::default(),
                stderr: Default::default(),
            });
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.contains("VGA") {
                let start = line.rfind("[").unwrap_or_default();
                let end = line.rfind("]").unwrap_or_default();
                return line.chars().skip(start).take(end - start + 1).collect();
            }
        }
        "Unknown Device".into()
    }

    /**
     * Unimplemented
     */
    #[cfg(not(target_os = "linux"))]
    pub fn get_gpu_brand(&self) -> String {
        "Unimplemented".into()
    }

    /**
     * Returns average CPU usage(percentage)
     */
    pub fn get_average_cpu_usage(&mut self) -> f32 {
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.sys.refresh_cpu_usage();
        (self.sys.global_cpu_usage() * 100f32).round() / 100f32
    }

    /**
     * Returns average GPU usage(percentage)
     */
    #[cfg(target_os = "linux")]
    pub fn get_average_gpu_usage(&self) -> f32 {
        // TODO: Reading from file is probably slow. so, we should use another GPU bindings.
        // TODO: 環境依存性を改善(/sys/class/drm/にcard1がある前提を改善する)
        std::fs::read_to_string("/sys/class/drm/card1/device/gpu_busy_percent")
            .unwrap_or_default()
            .strip_suffix("\n")
            .unwrap_or_default()
            .parse()
            .unwrap_or(0f32)
    }

    /**
     * Returns average GPU usage(unimplemented)
     */
    #[cfg(not(target_os = "linux"))]
    pub fn get_average_gpu_usage(&self) -> f32 {
        f32::default()
    }

    /**
     * Returns average CPU temp(℃)(unimplemented)
     */
    pub fn get_average_cpu_temp(&self) -> f32 {
        f32::default()
    }

    pub fn get_average_gpu_temp(&self) -> f32 {
        f32::default()
    }

    /**
     * Returns total memory size(GiB)
     */
    pub fn get_total_mem(&self) -> f32 {
        (self.sys.total_memory() as f32 * 100f32 / 1073741824f32).round() / 100f32
    }

    /**
     * Returns used memory size(GiB)
     */
    pub fn get_used_mem(&mut self) -> f32 {
        self.sys.refresh_memory();
        (self.sys.used_memory() as f32 * 100f32 / 1073741824f32).round() / 100f32
    }

    #[cfg(target_os = "linux")]
    pub fn get_total_vram(&self) -> f32 {
        // TODO: Reading from file is probably slow. so, we should use another GPU bindings.
        // TODO: 環境依存性を改善(/sys/class/drm/にcard1がある前提を改善する)
        (std::fs::read_to_string("/sys/class/drm/card1/device/mem_info_vram_total")
            .unwrap_or_default()
            .strip_suffix("\n")
            .unwrap_or_default()
            .parse()
            .unwrap_or(0f32)
            / 1073741824f32
            * 100f32)
            .round()
            / 100f32
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_total_vram(&self) -> f32 {
        f32::default()
    }

    #[cfg(target_os = "linux")]
    pub fn get_used_vram(&self) -> f32 {
        // TODO: Reading from file is probably slow. so, we should use another GPU bindings.
        // TODO: 環境依存性を改善(/sys/class/drm/にcard1がある前提を改善する)
        (std::fs::read_to_string("/sys/class/drm/card1/device/mem_info_vram_used")
            .unwrap_or_default()
            .strip_suffix("\n")
            .unwrap_or_default()
            .parse()
            .unwrap_or(0f32)
            / 1073741824f32
            * 100f32)
            .round()
            / 100f32
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_used_vram(&self) -> f32 {
        f32::default()
    }
}
