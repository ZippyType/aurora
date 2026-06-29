use axum::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct TemperatureInfo {
    pub zones: Vec<ZoneInfo>,
}

#[derive(Serialize)]
pub struct ZoneInfo {
    pub name: String,
    pub temp_c: f64,
    pub temp_millicelsius: u64,
}

pub async fn handler() -> Json<TemperatureInfo> {
    Json(get_temperature_info())
}

fn get_temperature_info() -> TemperatureInfo {
    let mut zones = Vec::new();
    let entries = fs::read_dir("/sys/class/thermal").unwrap_or_default();

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if !name_str.starts_with("thermal_zone") {
            continue;
        }

        let temp_path = path.join("temp");
        let type_path = path.join("type");

        let raw_temp = fs::read_to_string(&temp_path).ok();
        let zone_type = fs::read_to_string(&type_path).ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| name_str.to_string());

        if let Some(temp_str) = raw_temp {
            if let Ok(temp_mc) = temp_str.trim().parse::<u64>() {
                zones.push(ZoneInfo {
                    name: zone_type,
                    temp_c: temp_mc as f64 / 1000.0,
                    temp_millicelsius: temp_mc,
                });
            }
        }
    }

    TemperatureInfo { zones }
}
