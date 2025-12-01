pub mod collectors;
pub mod display;
pub mod cli;

pub use collectors::{SystemInfo, BatteryInfo, DisksInfo, NetworkInfo, TemperatureInfo, DisplayInfo};
pub use display::{OutputFormat, display_info};
