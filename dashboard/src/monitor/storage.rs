use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct StorageInfo {
    pub mounts: Vec<MountInfo>,
}

#[derive(Serialize)]
pub struct MountInfo {
    pub filesystem: String,
    pub mount_point: String,
    pub total_kb: u64,
    pub used_kb: u64,
    pub free_kb: u64,
    pub usage_percent: f64,
}

pub async fn handler() -> Json<StorageInfo> {
    Json(get_storage_info())
}

fn get_storage_info() -> StorageInfo {
    let mut mounts = Vec::new();
    let content = fs::read_to_string("/proc/mounts").unwrap_or_default();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let mount_point = parts[1].to_string();
        let stat = fs::read_to_string(format!("/sys/fs/{}/stat", &mount_point.trim_start_matches('/')));

        if let Ok(stat_content) = stat {
            let lines: Vec<&str> = stat_content.lines().collect();
            if lines.len() >= 3 {
                let total_kb: u64 = lines[0].parse().unwrap_or(0);
                let free_kb: u64 = lines[2].parse().unwrap_or(0);
                let used_kb = total_kb.saturating_sub(free_kb);
                let usage_percent = if total_kb > 0 {
                    (used_kb as f64 / total_kb as f64 * 100.0 * 100.0).round() / 100.0
                } else {
                    0.0
                };
                mounts.push(MountInfo {
                    filesystem: parts[0].to_string(),
                    mount_point,
                    total_kb,
                    used_kb,
                    free_kb,
                    usage_percent,
                });
            }
        } else {
            let statvfs = unsafe {
                let mut buf: libc::statvfs = std::mem::zeroed();
                let cpath = std::ffi::CString::new(mount_point.as_str()).unwrap();
                if libc::statvfs(cpath.as_ptr(), &mut buf) == 0 {
                    Some(buf)
                } else {
                    None
                }
            };
            if let Some(statvfs) = statvfs {
                let total_kb = (statvfs.f_blocks as u64 * statvfs.f_frsize as u64) / 1024;
                let free_kb = (statvfs.f_bfree as u64 * statvfs.f_frsize as u64) / 1024;
                let used_kb = total_kb.saturating_sub(free_kb);
                let usage_percent = if total_kb > 0 {
                    (used_kb as f64 / total_kb as f64 * 100.0 * 100.0).round() / 100.0
                } else {
                    0.0
                };
                mounts.push(MountInfo {
                    filesystem: parts[0].to_string(),
                    mount_point,
                    total_kb,
                    used_kb,
                    free_kb,
                    usage_percent,
                });
            }
        }
    }

    StorageInfo { mounts }
}
