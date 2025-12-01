use anyhow::Result;
use pico_args::Arguments;

pub struct CliArgs {
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_system: bool,
    pub show_battery: bool,
    pub show_disk: bool,
    pub show_network: bool,
    pub show_temp: bool,
    pub show_display: bool,
    pub show_all: bool,
    pub json: bool,
    pub watch: bool,
    pub interval: u64,
    pub help: bool,
}

impl CliArgs {
    pub fn parse() -> Result<Self> {
        let mut args = Arguments::from_env();

        let help = args.contains(["-h", "--help"]);

        if help {
            return Ok(Self {
                show_cpu: false,
                show_memory: false,
                show_system: false,
                show_battery: false,
                show_disk: false,
                show_network: false,
                show_temp: false,
                show_display: false,
                show_all: false,
                json: false,
                watch: false,
                interval: 2,
                help: true,
            });
        }

        let show_cpu = args.contains("--cpu");
        let show_memory = args.contains("--memory");
        let show_system = args.contains("--system");
        let show_battery = args.contains("--battery");
        let show_disk = args.contains("--disk");
        let show_network = args.contains("--network");
        let show_temp = args.contains("--temp");
        let show_display = args.contains("--display");
        let json = args.contains("--json");
        let watch = args.contains(["-w", "--watch"]);
        let interval: u64 = args.opt_value_from_str(["-i", "--interval"])?.unwrap_or(2);

        let show_all = !show_cpu && !show_memory && !show_system && !show_battery && !show_disk && !show_network && !show_temp && !show_display;

        let remaining = args.finish();
        if !remaining.is_empty() {
            anyhow::bail!("Unknown arguments: {:?}", remaining);
        }

        Ok(Self {
            show_cpu,
            show_memory,
            show_system,
            show_battery,
            show_disk,
            show_network,
            show_temp,
            show_display,
            show_all,
            json,
            watch,
            interval,
            help,
        })
    }

    pub fn print_help() {
        println!(
            r#"weni - Lightweight cross-platform system information tool

USAGE:
    weni [OPTIONS]

OPTIONS:
    --cpu                 Show CPU information
    --memory              Show memory information
    --system              Show system information
    --battery             Show battery information
    --disk                Show disk information
    --network             Show network information
    --temp                Show temperature information
    --display             Show display/monitor information
    --json                Output in JSON format
    -w, --watch           Enable watch mode (live updates)
    -i, --interval <SEC>  Update interval in seconds (default: 2)
    -h, --help            Print help information

EXAMPLES:
    weni                        # Show all information
    weni --cpu --memory         # Show only CPU and memory
    weni --display              # Show monitor information
    weni --json                 # Output all info as JSON
    weni --watch                # Live monitoring mode
    weni --watch --interval 5   # Monitor with 5 second interval
"#
        );
    }
}
