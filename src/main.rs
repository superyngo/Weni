use anyhow::Result;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use weni::{
    BatteryInfo, DisksInfo, NetworkInfo, SystemInfo, TemperatureInfo, DisplayInfo,
    cli::CliArgs,
    display::{display_info, OutputFormat},
};

fn main() -> Result<()> {
    let args = CliArgs::parse()?;

    if args.help {
        CliArgs::print_help();
        return Ok(());
    }

    if args.watch {
        run_watch_mode(args)
    } else {
        run_once(args)
    }
}

fn run_once(args: CliArgs) -> Result<()> {
    let (system_info, battery_info, disks_info, network_info, temp_info, display_info_data) = collect_info(&args);

    let format = if args.json {
        OutputFormat::Json
    } else {
        OutputFormat::Text
    };

    display_info(system_info, battery_info, disks_info, network_info, temp_info, display_info_data, format)?;

    Ok(())
}

fn run_watch_mode(args: CliArgs) -> Result<()> {
    if args.json {
        anyhow::bail!("Watch mode is not compatible with JSON output");
    }

    loop {
        clear_screen();
        let (system_info, battery_info, disks_info, network_info, temp_info, display_info_data) = collect_info(&args);
        display_info(
            system_info,
            battery_info,
            disks_info,
            network_info,
            temp_info,
            display_info_data,
            OutputFormat::Text,
        )?;
        println!("Press Ctrl+C to exit | Refreshing every {} seconds", args.interval);
        io::stdout().flush()?;
        thread::sleep(Duration::from_secs(args.interval));
    }
}

fn collect_info(args: &CliArgs) -> (SystemInfo, Option<BatteryInfo>, Option<DisksInfo>, Option<NetworkInfo>, Option<TemperatureInfo>, Option<DisplayInfo>) {
    let collect_cpu = args.show_all || args.show_cpu;
    let collect_memory = args.show_all || args.show_memory;
    let collect_system = args.show_all || args.show_system;
    let collect_battery = args.show_all || args.show_battery;
    let collect_disk = args.show_all || args.show_disk;
    let collect_network = args.show_all || args.show_network;
    let collect_temp = args.show_all || args.show_temp;
    let collect_display = args.show_all || args.show_display;

    let system_info = SystemInfo::new(collect_cpu, collect_memory, collect_system);

    let battery_info = if collect_battery {
        match BatteryInfo::collect() {
            Ok(info) => info,
            Err(_) => None,
        }
    } else {
        None
    };

    let disks_info = if collect_disk {
        Some(DisksInfo::collect())
    } else {
        None
    };

    let network_info = if collect_network {
        Some(NetworkInfo::collect())
    } else {
        None
    };

    let temp_info = if collect_temp {
        Some(TemperatureInfo::collect())
    } else {
        None
    };

    let display_info_data = if collect_display {
        Some(DisplayInfo::collect())
    } else {
        None
    };

    (system_info, battery_info, disks_info, network_info, temp_info, display_info_data)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
