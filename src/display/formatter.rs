use anyhow::Result;
use colored::Colorize;
use comfy_table::{Table, Row, Cell, presets::UTF8_FULL};
use serde::Serialize;

use crate::collectors::{SystemInfo, BatteryInfo, DisksInfo, NetworkInfo, TemperatureInfo, DisplayInfo};

pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Serialize)]
struct AllInfo {
    system: SystemInfo,
    battery: Option<BatteryInfo>,
    disks: Option<DisksInfo>,
    network: Option<NetworkInfo>,
    temperature: Option<TemperatureInfo>,
    display: Option<DisplayInfo>,
}

pub fn display_info(
    system_info: SystemInfo,
    battery_info: Option<BatteryInfo>,
    disks_info: Option<DisksInfo>,
    network_info: Option<NetworkInfo>,
    temp_info: Option<TemperatureInfo>,
    display_info: Option<DisplayInfo>,
    format: OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Text => display_text(&system_info, &battery_info, &disks_info, &network_info, &temp_info, &display_info),
        OutputFormat::Json => display_json(&system_info, &battery_info, &disks_info, &network_info, &temp_info, &display_info),
    }
}

fn display_text(
    system_info: &SystemInfo,
    battery_info: &Option<BatteryInfo>,
    disks_info: &Option<DisksInfo>,
    network_info: &Option<NetworkInfo>,
    temp_info: &Option<TemperatureInfo>,
    display_info: &Option<DisplayInfo>,
) -> Result<()> {

    if let Some(os) = &system_info.os {
        println!("\n{}", "System Information".bold().cyan());
        let mut sys_table = Table::new();
        sys_table.load_preset(UTF8_FULL);
        sys_table.add_row(Row::from(vec![
            Cell::new("OS").fg(comfy_table::Color::Yellow),
            Cell::new(&os.name),
        ]));
        sys_table.add_row(Row::from(vec![
            Cell::new("Version").fg(comfy_table::Color::Yellow),
            Cell::new(&os.os_version),
        ]));
        sys_table.add_row(Row::from(vec![
            Cell::new("Kernel").fg(comfy_table::Color::Yellow),
            Cell::new(&os.kernel_version),
        ]));
        sys_table.add_row(Row::from(vec![
            Cell::new("Hostname").fg(comfy_table::Color::Yellow),
            Cell::new(&os.hostname),
        ]));
        println!("{sys_table}");
    }

    if let Some(cpu) = &system_info.cpu {
        println!("\n{}", "CPU Information".bold().cyan());
        let mut cpu_table = Table::new();
        cpu_table.load_preset(UTF8_FULL);
        cpu_table.add_row(Row::from(vec![
            Cell::new("Model").fg(comfy_table::Color::Yellow),
            Cell::new(&cpu.name),
        ]));
        cpu_table.add_row(Row::from(vec![
            Cell::new("Cores").fg(comfy_table::Color::Yellow),
            Cell::new(cpu.cores.to_string()),
        ]));
        cpu_table.add_row(Row::from(vec![
            Cell::new("Usage").fg(comfy_table::Color::Yellow),
            Cell::new(format!("{:.2}%", cpu.usage)),
        ]));
        cpu_table.add_row(Row::from(vec![
            Cell::new("Frequency").fg(comfy_table::Color::Yellow),
            Cell::new(format!("{} MHz", cpu.frequency)),
        ]));
        println!("{cpu_table}");
    }

    if let Some(memory) = &system_info.memory {
        println!("\n{}", "Memory Information".bold().cyan());
        let mut mem_table = Table::new();
        mem_table.load_preset(UTF8_FULL);
        mem_table.add_row(Row::from(vec![
            Cell::new("Total").fg(comfy_table::Color::Yellow),
            Cell::new(format_bytes(memory.total)),
        ]));
        mem_table.add_row(Row::from(vec![
            Cell::new("Used").fg(comfy_table::Color::Yellow),
            Cell::new(format_bytes(memory.used)),
        ]));
        mem_table.add_row(Row::from(vec![
            Cell::new("Available").fg(comfy_table::Color::Yellow),
            Cell::new(format_bytes(memory.available)),
        ]));
        mem_table.add_row(Row::from(vec![
            Cell::new("Usage").fg(comfy_table::Color::Yellow),
            Cell::new(format!("{:.2}%", memory.usage_percent)),
        ]));
        println!("{mem_table}");
    }

    if let Some(battery) = battery_info {
        println!("\n{}", "Battery Information".bold().cyan());
        let mut bat_table = Table::new();
        bat_table.load_preset(UTF8_FULL);
        bat_table.add_row(Row::from(vec![
            Cell::new("State").fg(comfy_table::Color::Yellow),
            Cell::new(&battery.state),
        ]));
        bat_table.add_row(Row::from(vec![
            Cell::new("Charge").fg(comfy_table::Color::Yellow),
            Cell::new(format!("{:.2}%", battery.percentage)),
        ]));
        if let Some(ref time) = battery.time_to_full {
            bat_table.add_row(Row::from(vec![
                Cell::new("Time to Full").fg(comfy_table::Color::Yellow),
                Cell::new(time),
            ]));
        }
        if let Some(ref time) = battery.time_to_empty {
            bat_table.add_row(Row::from(vec![
                Cell::new("Time to Empty").fg(comfy_table::Color::Yellow),
                Cell::new(time),
            ]));
        }
        bat_table.add_row(Row::from(vec![
            Cell::new("Health").fg(comfy_table::Color::Yellow),
            Cell::new(format!("{:.2}%", battery.health)),
        ]));
        bat_table.add_row(Row::from(vec![
            Cell::new("Technology").fg(comfy_table::Color::Yellow),
            Cell::new(&battery.technology),
        ]));
        if let Some(temp) = battery.temperature {
            bat_table.add_row(Row::from(vec![
                Cell::new("Temperature").fg(comfy_table::Color::Yellow),
                Cell::new(format!("{:.1}°C", temp)),
            ]));
        }
        println!("{bat_table}");
    }

    if let Some(disks) = disks_info {
        println!("\n{}", "Disk Information".bold().cyan());
        for disk in &disks.disks {
            let mut disk_table = Table::new();
            disk_table.load_preset(UTF8_FULL);
            disk_table.add_row(Row::from(vec![
                Cell::new("Mount Point").fg(comfy_table::Color::Yellow),
                Cell::new(&disk.mount_point),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("Name").fg(comfy_table::Color::Yellow),
                Cell::new(&disk.name),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("File System").fg(comfy_table::Color::Yellow),
                Cell::new(&disk.file_system),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("Total").fg(comfy_table::Color::Yellow),
                Cell::new(format_bytes(disk.total_space)),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("Used").fg(comfy_table::Color::Yellow),
                Cell::new(format_bytes(disk.used_space)),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("Available").fg(comfy_table::Color::Yellow),
                Cell::new(format_bytes(disk.available_space)),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("Usage").fg(comfy_table::Color::Yellow),
                Cell::new(format!("{:.2}%", disk.usage_percent)),
            ]));
            disk_table.add_row(Row::from(vec![
                Cell::new("Removable").fg(comfy_table::Color::Yellow),
                Cell::new(if disk.is_removable { "Yes" } else { "No" }),
            ]));
            println!("{disk_table}\n");
        }
    }

    if let Some(network) = network_info {
        println!("{}", "Network Information".bold().cyan());
        for iface in &network.interfaces {
            let mut net_table = Table::new();
            net_table.load_preset(UTF8_FULL);
            net_table.add_row(Row::from(vec![
                Cell::new("Interface").fg(comfy_table::Color::Yellow),
                Cell::new(&iface.name),
            ]));
            net_table.add_row(Row::from(vec![
                Cell::new("Received").fg(comfy_table::Color::Yellow),
                Cell::new(format_bytes(iface.received)),
            ]));
            net_table.add_row(Row::from(vec![
                Cell::new("Transmitted").fg(comfy_table::Color::Yellow),
                Cell::new(format_bytes(iface.transmitted)),
            ]));
            net_table.add_row(Row::from(vec![
                Cell::new("Packets RX").fg(comfy_table::Color::Yellow),
                Cell::new(iface.packets_received.to_string()),
            ]));
            net_table.add_row(Row::from(vec![
                Cell::new("Packets TX").fg(comfy_table::Color::Yellow),
                Cell::new(iface.packets_transmitted.to_string()),
            ]));
            net_table.add_row(Row::from(vec![
                Cell::new("Errors RX").fg(comfy_table::Color::Yellow),
                Cell::new(iface.errors_received.to_string()),
            ]));
            net_table.add_row(Row::from(vec![
                Cell::new("Errors TX").fg(comfy_table::Color::Yellow),
                Cell::new(iface.errors_transmitted.to_string()),
            ]));
            println!("{net_table}\n");
        }
    }

    if let Some(temp) = temp_info {
        if !temp.components.is_empty() {
            println!("{}", "Temperature Information".bold().cyan());
            for component in &temp.components {
                let mut temp_table = Table::new();
                temp_table.load_preset(UTF8_FULL);
                temp_table.add_row(Row::from(vec![
                    Cell::new("Component").fg(comfy_table::Color::Yellow),
                    Cell::new(&component.label),
                ]));
                temp_table.add_row(Row::from(vec![
                    Cell::new("Temperature").fg(comfy_table::Color::Yellow),
                    Cell::new(format!("{:.1}°C", component.temperature)),
                ]));
                if let Some(max) = component.max {
                    temp_table.add_row(Row::from(vec![
                        Cell::new("Max").fg(comfy_table::Color::Yellow),
                        Cell::new(format!("{:.1}°C", max)),
                    ]));
                }
                if let Some(critical) = component.critical {
                    temp_table.add_row(Row::from(vec![
                        Cell::new("Critical").fg(comfy_table::Color::Yellow),
                        Cell::new(format!("{:.1}°C", critical)),
                    ]));
                }
                println!("{temp_table}\n");
            }
        }
    }

    if let Some(displays) = display_info {
        if !displays.monitors.is_empty() {
            println!("{}", "Display Information".bold().cyan());
            println!("Total Monitors: {}\n", displays.total_count);

            for monitor in &displays.monitors {
                let mut display_table = Table::new();
                display_table.load_preset(UTF8_FULL);
                display_table.add_row(Row::from(vec![
                    Cell::new("Monitor").fg(comfy_table::Color::Yellow),
                    Cell::new(if monitor.is_primary {
                        format!("{} (Primary)", monitor.name)
                    } else {
                        monitor.name.clone()
                    }),
                ]));
                display_table.add_row(Row::from(vec![
                    Cell::new("ID").fg(comfy_table::Color::Yellow),
                    Cell::new(monitor.id.to_string()),
                ]));
                display_table.add_row(Row::from(vec![
                    Cell::new("Resolution").fg(comfy_table::Color::Yellow),
                    Cell::new(format!("{}x{}", monitor.width, monitor.height)),
                ]));
                display_table.add_row(Row::from(vec![
                    Cell::new("Refresh Rate").fg(comfy_table::Color::Yellow),
                    Cell::new(format!("{} Hz", monitor.refresh_rate)),
                ]));
                display_table.add_row(Row::from(vec![
                    Cell::new("Scale Factor").fg(comfy_table::Color::Yellow),
                    Cell::new(format!("{:.2}x", monitor.scale_factor)),
                ]));
                display_table.add_row(Row::from(vec![
                    Cell::new("Position").fg(comfy_table::Color::Yellow),
                    Cell::new(format!("({}, {})", monitor.x, monitor.y)),
                ]));
                println!("{display_table}\n");
            }
        }
    }

    println!();
    Ok(())
}

fn display_json(
    system_info: &SystemInfo,
    battery_info: &Option<BatteryInfo>,
    disks_info: &Option<DisksInfo>,
    network_info: &Option<NetworkInfo>,
    temp_info: &Option<TemperatureInfo>,
    display_info: &Option<DisplayInfo>,
) -> Result<()> {
    let all_info = AllInfo {
        system: system_info.clone(),
        battery: battery_info.clone(),
        disks: disks_info.clone(),
        network: network_info.clone(),
        temperature: temp_info.clone(),
        display: display_info.clone(),
    };

    let json = serde_json::to_string_pretty(&all_info)?;
    println!("{}", json);
    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
