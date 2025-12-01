use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BatteryInfo {
    pub state: String,
    pub percentage: f32,
    pub time_to_full: Option<String>,
    pub time_to_empty: Option<String>,
    pub health: f32,
    pub technology: String,
    pub temperature: Option<f32>,
}

// Battery functionality is disabled on i686-pc-windows-msvc due to battery crate compilation issues
#[cfg(all(target_os = "windows", target_arch = "x86"))]
impl BatteryInfo {
    pub fn collect() -> Result<Option<Self>> {
        // Battery crate doesn't compile on i686-pc-windows-msvc
        Ok(None)
    }
}

#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
impl BatteryInfo {
    pub fn collect() -> Result<Option<Self>> {
        use battery::{Manager, State};
        
        let manager = Manager::new()?;

        let batteries: Vec<_> = manager.batteries()?.collect::<Result<Vec<_>, _>>()?;

        if batteries.is_empty() {
            return Ok(None);
        }

        let battery = &batteries[0];

        let state = match battery.state() {
            State::Charging => "Charging",
            State::Discharging => "Discharging",
            State::Full => "Full",
            State::Empty => "Empty",
            _ => "Unknown",
        }
        .to_string();

        let percentage = battery.state_of_charge().get::<battery::units::ratio::percent>();

        let time_to_full = battery.time_to_full().map(|duration| {
            let secs = duration.get::<battery::units::time::second>() as u64;
            Self::format_duration(secs)
        });

        let time_to_empty = battery.time_to_empty().map(|duration| {
            let secs = duration.get::<battery::units::time::second>() as u64;
            Self::format_duration(secs)
        });

        let health = battery.state_of_health().get::<battery::units::ratio::percent>();

        let technology = format!("{:?}", battery.technology());

        let temperature = battery.temperature().map(|t| {
            t.get::<battery::units::thermodynamic_temperature::degree_celsius>()
        });

        Ok(Some(Self {
            state,
            percentage,
            time_to_full,
            time_to_empty,
            health,
            technology,
            temperature,
        }))
    }

    fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}
