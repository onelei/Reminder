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
    pub work_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    pub long_break_interval: u32,
    pub long_break_mode: bool,
    pub strict_break_mode: bool,
    pub auto_break_mode: bool,
    pub daily_focus_goal: bool,
    pub daily_goal_count: u32,
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
            work_minutes: 25,
            short_break_minutes: 5,
            long_break_minutes: 15,
            long_break_interval: 4,
            long_break_mode: false,
            strict_break_mode: false,
            auto_break_mode: false,
            daily_focus_goal: false,
            daily_goal_count: 8,
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

    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_config_to_disk(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
