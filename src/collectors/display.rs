use serde::Serialize;
use display_info::DisplayInfo as DisplayInfoCrate;

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DisplayInfo {
    pub monitors: Vec<MonitorInfo>,
    pub total_count: usize,
}

impl DisplayInfo {
    pub fn collect() -> Self {
        let displays = DisplayInfoCrate::all().unwrap_or_default();

        let monitors: Vec<MonitorInfo> = displays
            .iter()
            .map(|display| MonitorInfo {
                id: display.id,
                name: display.name.clone(),
                width: display.width,
                height: display.height,
                refresh_rate: display.frequency as u32,
                scale_factor: display.scale_factor,
                is_primary: display.is_primary,
                x: display.x,
                y: display.y,
            })
            .collect();

        let total_count = monitors.len();

        Self {
            monitors,
            total_count,
        }
    }
}
