use serde::Serialize;
use sysinfo::Disks;

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub used_space: u64,
    pub usage_percent: f32,
    pub file_system: String,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DisksInfo {
    pub disks: Vec<DiskInfo>,
}

impl DisksInfo {
    pub fn collect() -> Self {
        let disks = Disks::new_with_refreshed_list();
        let disk_list: Vec<DiskInfo> = disks
            .iter()
            .map(|disk| {
                let total = disk.total_space();
                let available = disk.available_space();
                let used = total - available;
                let usage_percent = if total > 0 {
                    (used as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                DiskInfo {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total_space: total,
                    available_space: available,
                    used_space: used,
                    usage_percent,
                    file_system: disk.file_system().to_string_lossy().to_string(),
                    is_removable: disk.is_removable(),
                }
            })
            .collect();

        Self { disks: disk_list }
    }
}
