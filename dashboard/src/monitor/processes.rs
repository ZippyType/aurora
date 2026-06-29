use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct ProcessList {
    pub processes: Vec<ProcessInfo>,
    pub total: usize,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: String,
    pub cpu_percent: f64,
    pub mem_kb: u64,
    pub user: String,
}

pub async fn handler() -> Json<ProcessList> {
    Json(get_processes())
}

fn get_processes() -> ProcessList {
    let mut processes = Vec::new();
    let entries = fs::read_dir("/proc").unwrap_or_default();

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let pid: u32 = match name_str.parse() {
            Ok(p) if p > 0 => p,
            _ => continue,
        };

        let stat_path = entry.path().join("stat");
        let status_path = entry.path().join("status");
        let stat_content = fs::read_to_string(&stat_path).unwrap_or_default();
        let status_content = fs::read_to_string(&status_path).unwrap_or_default();

        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() < 15 {
            continue;
        }

        let name_part = stat_content
            .find('(')
            .and_then(|start| {
                stat_content[start..].rfind(')').map(|end| &stat_content[start + 1..start + end])
            })
            .unwrap_or("")
            .to_string();

        let state = fields.get(2).unwrap_or(&"?").to_string();

        let utime: u64 = fields.get(13).and_then(|s| s.parse().ok()).unwrap_or(0);
        let stime: u64 = fields.get(14).and_then(|s| s.parse().ok()).unwrap_or(0);

        let total_time = (utime + stime) as f64;
        let cpu_percent = if total_time > 0.0 {
            (total_time / 100.0 * 100.0).round() / 100.0
        } else {
            0.0
        };

        let mut mem_kb = 0u64;
        let mut user = String::new();
        for status_line in status_content.lines() {
            if let Some(val) = status_line.strip_prefix("VmRSS:") {
                mem_kb = val.trim().trim_end_matches(" kB").parse().unwrap_or(0);
            }
            if let Some(val) = status_line.strip_prefix("Uid:") {
                let uid: u32 = val.split_whitespace().next().and_then(|s| s.parse()).unwrap_or(0);
                user = uid_to_name(uid);
            }
        }

        processes.push(ProcessInfo {
            pid,
            name: name_part,
            state,
            cpu_percent,
            mem_kb,
            user,
        });
    }

    processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal));
    let total = processes.len();
    processes.truncate(100);

    ProcessList { processes, total }
}

fn uid_to_name(uid: u32) -> String {
    fs::read_to_string("/etc/passwd").unwrap_or_default()
        .lines()
        .find_map(|line| {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() >= 2 {
                let uid_str = parts[2].trim_end_matches(':');
                if uid_str.parse::<u32>().ok() == Some(uid) {
                    return Some(parts[0].to_string());
                }
            }
            None
        })
        .unwrap_or_else(|| uid.to_string())
}
