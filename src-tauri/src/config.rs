use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AppLocale {
    #[default]
    Zh,
    En,
}
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

pub const DEFAULT_THEME: &str = "#2aab9b";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub break_minutes: u32,
    pub custom_break_background: bool,
    pub break_background_path: Option<String>,
    pub custom_theme_color: bool,
    pub theme_color: String,
    pub autostart: bool,
    #[serde(default)]
    pub locale: AppLocale,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            break_minutes: 5,
            custom_break_background: false,
            break_background_path: None,
            custom_theme_color: true,
            theme_color: DEFAULT_THEME.to_string(),
            autostart: false,
            locale: AppLocale::default(),
        }
    }
}

pub fn is_english(locale: AppLocale) -> bool {
    matches!(locale, AppLocale::En)
}

pub type SharedConfig = Arc<Mutex<AppConfig>>;

pub fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|e| e.to_string())
        .map(|p| p.join("config.json"))
}

pub fn load_config(app: &AppHandle) -> AppConfig {
    let path = match config_path(app) {
        Ok(p) => p,
        Err(_) => return AppConfig::default(),
    };

    if !path.exists() {
        return AppConfig::default();
    }

    let Ok(raw) = fs::read_to_string(&path) else {
        return AppConfig::default();
    };

    let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return AppConfig::default();
    };

    let Some(obj) = value.as_object() else {
        return AppConfig::default();
    };

    let mut cfg = AppConfig::default();

    if let Some(v) = obj.get("breakMinutes").and_then(|v| v.as_u64()) {
        cfg.break_minutes = v as u32;
    }

    if let Some(v) = obj.get("customBreakBackground").and_then(|v| v.as_bool()) {
        cfg.custom_break_background = v;
    }

    if let Some(v) = obj.get("breakBackgroundPath") {
        cfg.break_background_path = v.as_str().map(String::from);
    }

    if let Some(v) = obj.get("customThemeColor").and_then(|v| v.as_bool()) {
        cfg.custom_theme_color = v;
    }

    if let Some(v) = obj.get("themeColor").and_then(|v| v.as_str()) {
        cfg.theme_color = v.to_string();
    }

    if let Some(v) = obj.get("autostart").and_then(|v| v.as_bool()) {
        cfg.autostart = v;
    }

    if let Some(v) = obj.get("locale").and_then(|v| v.as_str()) {
        cfg.locale = if v.eq_ignore_ascii_case("en") {
            AppLocale::En
        } else {
            AppLocale::Zh
        };
    }

    if cfg.break_minutes == 0 {
        cfg.break_minutes = AppConfig::default().break_minutes;
    }

    cfg
}

pub fn save_config_to_disk(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
