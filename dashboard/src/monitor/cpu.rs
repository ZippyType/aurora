use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct CpuInfo {
    pub usage_percent: f64,
    pub cores: Vec<CoreInfo>,
    pub load_avg: [f64; 3],
}

#[derive(Serialize)]
pub struct CoreInfo {
    pub core_id: u32,
    pub usage_percent: f64,
}

pub async fn handler() -> Json<CpuInfo> {
    Json(get_cpu_info())
}

fn parse_cpu_line(line: &str) -> Option<(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 11 {
        return None;
    }
    let nums: Vec<u64> = parts[1..11].iter().filter_map(|s| s.parse().ok()).collect();
    if nums.len() < 10 {
        return None;
    }
    Some((nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], nums[8], nums[9]))
}

fn get_cpu_info() -> CpuInfo {
    let stat = fs::read_to_string("/proc/stat").unwrap_or_default();
    let mut cores = Vec::new();
    let mut total_idle = 0u64;
    let mut total_all = 0u64;

    for line in stat.lines() {
        if line.starts_with("cpu") && !line.starts_with("cpu ") {
            let id_str = &line[3..];
            let id_end = id_str.find(char::is_whitespace).unwrap_or(id_str.len());
            let core_id: u32 = id_str[..id_end].parse().unwrap_or(0);

            if let Some((user, nice, sys, idle, iowait, irq, softirq, steal, _, _)) = parse_cpu_line(line) {
                let total = user + nice + sys + idle + iowait + irq + softirq + steal;
                let usage = if total > 0 {
                    100.0 * (1.0 - idle as f64 / total as f64)
                } else {
                    0.0
                };
                cores.push(CoreInfo { core_id, usage_percent: (usage * 100.0).round() / 100.0 });
                total_idle += idle + iowait;
                total_all += total;
            }
        }
    }

    let total_usage = if total_all > 0 {
        100.0 * (1.0 - total_idle as f64 / total_all as f64)
    } else {
        0.0
    };

    let load_raw = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let load_parts: Vec<f64> = load_raw
        .split_whitespace()
        .take(3)
        .filter_map(|s| s.parse().ok())
        .collect();
    let load_avg = match load_parts.len() {
        3 => [load_parts[0], load_parts[1], load_parts[2]],
        _ => [0.0, 0.0, 0.0],
    };

    CpuInfo {
        usage_percent: (total_usage * 100.0).round() / 100.0,
        cores,
        load_avg,
    }
}
