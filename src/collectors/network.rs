use serde::Serialize;
use sysinfo::Networks;

#[derive(Debug, Clone, Serialize)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub errors_received: u64,
    pub errors_transmitted: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterfaceInfo>,
}

impl NetworkInfo {
    pub fn collect() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let interfaces: Vec<NetworkInterfaceInfo> = networks
            .iter()
            .map(|(name, data)| NetworkInterfaceInfo {
                name: name.to_string(),
                received: data.total_received(),
                transmitted: data.total_transmitted(),
                packets_received: data.total_packets_received(),
                packets_transmitted: data.total_packets_transmitted(),
                errors_received: data.total_errors_on_received(),
                errors_transmitted: data.total_errors_on_transmitted(),
            })
            .collect();

        Self { interfaces }
    }

    pub fn refresh(&mut self) {
        let mut networks = Networks::new_with_refreshed_list();
        std::thread::sleep(std::time::Duration::from_millis(100));
        networks.refresh();

        self.interfaces = networks
            .iter()
            .map(|(name, data)| NetworkInterfaceInfo {
                name: name.to_string(),
                received: data.total_received(),
                transmitted: data.total_transmitted(),
                packets_received: data.total_packets_received(),
                packets_transmitted: data.total_packets_transmitted(),
                errors_received: data.total_errors_on_received(),
                errors_transmitted: data.total_errors_on_transmitted(),
            })
            .collect();
    }
}
