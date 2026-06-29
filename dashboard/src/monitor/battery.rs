use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct BatteryInfo {
    pub present: bool,
    pub capacity: Option<u8>,
    pub status: Option<String>,
    pub voltage_uv: Option<u64>,
    pub current_ua: Option<i64>,
    pub temperature_c: Option<f64>,
}

pub async fn handler() -> Json<BatteryInfo> {
    Json(get_battery_info())
}

fn read_sysfs(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_sysfs_u64(path: &str) -> Option<u64> {
    read_sysfs(path).and_then(|s| s.parse().ok())
}

fn read_sysfs_i64(path: &str) -> Option<i64> {
    read_sysfs(path).and_then(|s| s.parse().ok())
}

fn find_power_supply() -> Option<String> {
    let entries = fs::read_dir("/sys/class/power_supply").ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with("mc") || name_str.starts_with("bq") || name_str.starts_with("max") {
            let uevent_path = entry.path().join("uevent");
            if uevent_path.exists() {
                return Some(name_str.to_string());
            }
        }
    }
    Some("battery".to_string())
}

fn get_battery_info() -> BatteryInfo {
    let supply_name = match find_power_supply() {
        Some(n) => n,
        None => {
            return BatteryInfo {
                present: false,
                capacity: None,
                status: None,
                voltage_uv: None,
                current_ua: None,
                temperature_c: None,
            };
        }
    };

    let base = format!("/sys/class/power_supply/{supply_name}");
    let present = read_sysfs(&format!("{base}/present"))
        .map(|s| s == "1")
        .unwrap_or(false);

    let capacity = read_sysfs_u64(&format!("{base}/capacity")).map(|c| c as u8);
    let status = read_sysfs(&format!("{base}/status"));
    let voltage_uv = read_sysfs_u64(&format!("{base}/voltage_now"));
    let current_ua = read_sysfs_i64(&format!("{base}/current_now"));
    let temperature_c = read_sysfs_u64(&format!("{base}/temp"))
        .map(|t| (t as f64 / 10.0 * 100.0).round() / 100.0);

    BatteryInfo {
        present,
        capacity,
        status,
        voltage_uv,
        current_ua,
        temperature_c,
    }
}
