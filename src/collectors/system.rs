use serde::Serialize;
use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

#[derive(Debug, Clone, Serialize)]
pub struct CpuInfo {
    pub name: String,
    pub cores: usize,
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct OsInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub cpu: Option<CpuInfo>,
    pub memory: Option<MemoryInfo>,
    pub os: Option<OsInfo>,
}

impl SystemInfo {
    pub fn new(collect_cpu: bool, collect_memory: bool, collect_os: bool) -> Self {
        let mut refresh_kind = RefreshKind::new();

        if collect_cpu {
            refresh_kind = refresh_kind.with_cpu(CpuRefreshKind::everything());
        }
        if collect_memory {
            refresh_kind = refresh_kind.with_memory(MemoryRefreshKind::everything());
        }

        let mut sys = System::new_with_specifics(refresh_kind);

        if collect_cpu {
            std::thread::sleep(std::time::Duration::from_millis(200));
            sys.refresh_cpu_all();
        }

        let cpu = if collect_cpu {
            Some(Self::collect_cpu_info(&sys))
        } else {
            None
        };

        let memory = if collect_memory {
            Some(Self::collect_memory_info(&sys))
        } else {
            None
        };

        let os = if collect_os {
            Some(Self::collect_os_info(&sys))
        } else {
            None
        };

        Self { cpu, memory, os }
    }

    fn collect_cpu_info(sys: &System) -> CpuInfo {
        let cpus = sys.cpus();
        let physical_cores = sys.physical_core_count().unwrap_or(0);
        let cpu_name = if !cpus.is_empty() {
            cpus[0].brand().to_string()
        } else {
            "Unknown".to_string()
        };
        let cpu_frequency = if !cpus.is_empty() {
            cpus[0].frequency()
        } else {
            0
        };

        CpuInfo {
            name: cpu_name,
            cores: physical_cores,
            usage: sys.global_cpu_usage(),
            frequency: cpu_frequency,
        }
    }

    fn collect_memory_info(sys: &System) -> MemoryInfo {
        let total = sys.total_memory();
        let available = sys.available_memory();
        let used = total - available;
        let usage_percent = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        MemoryInfo {
            total,
            used,
            available,
            usage_percent,
        }
    }

    fn collect_os_info(_sys: &System) -> OsInfo {
        OsInfo {
            name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        }
    }
}
