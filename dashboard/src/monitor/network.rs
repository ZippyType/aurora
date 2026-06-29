use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<InterfaceInfo>,
}

#[derive(Serialize)]
pub struct InterfaceInfo {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub is_up: bool,
}

pub async fn handler() -> Json<NetworkInfo> {
    Json(get_network_info())
}

fn get_network_info() -> NetworkInfo {
    let mut interfaces = Vec::new();
    let content = fs::read_to_string("/proc/net/dev").unwrap_or_default();

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            continue;
        }

        let name = parts[0].trim_end_matches(':').to_string();
        if name == "lo" {
            continue;
        }

        let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
        let rx_packets: u64 = parts[2].parse().unwrap_or(0);
        let rx_errors: u64 = parts[3].parse().unwrap_or(0);
        let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
        let tx_packets: u64 = parts[10].parse().unwrap_or(0);
        let tx_errors: u64 = parts[11].parse().unwrap_or(0);

        let carrier_path = format!("/sys/class/net/{name}/carrier");
        let is_up = fs::read_to_string(&carrier_path)
            .ok()
            .map(|s| s.trim() == "1")
            .unwrap_or(false);

        interfaces.push(InterfaceInfo {
            name,
            rx_bytes,
            tx_bytes,
            rx_packets,
            tx_packets,
            rx_errors,
            tx_errors,
            is_up,
        });
    }

    NetworkInfo { interfaces }
}
