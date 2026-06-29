use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_kb: u64,
    pub free_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub usage_percent: f64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
    pub swap_used_kb: u64,
}

pub async fn handler() -> Json<MemoryInfo> {
    Json(get_memory_info())
}

fn parse_meminfo() -> std::collections::HashMap<String, u64> {
    let content = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut map = std::collections::HashMap::new();
    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let val_str = parts[1].trim().replace(" kB", "");
            if let Ok(val) = val_str.parse::<u64>() {
                map.insert(key, val);
            }
        }
    }
    map
}

fn get_memory_info() -> MemoryInfo {
    let m = parse_meminfo();
    let total_kb = m.get("MemTotal").copied().unwrap_or(0);
    let free_kb = m.get("MemFree").copied().unwrap_or(0);
    let available_kb = m.get("MemAvailable").copied().unwrap_or(0);
    let used_kb = total_kb.saturating_sub(free_kb);
    let usage_percent = if total_kb > 0 {
        ((total_kb - available_kb) as f64 / total_kb as f64 * 100.0 * 100.0).round() / 100.0
    } else {
        0.0
    };
    let swap_total_kb = m.get("SwapTotal").copied().unwrap_or(0);
    let swap_free_kb = m.get("SwapFree").copied().unwrap_or(0);
    let swap_used_kb = swap_total_kb.saturating_sub(swap_free_kb);

    MemoryInfo {
        total_kb,
        free_kb,
        available_kb,
        used_kb,
        usage_percent,
        swap_total_kb,
        swap_free_kb,
        swap_used_kb,
    }
}
