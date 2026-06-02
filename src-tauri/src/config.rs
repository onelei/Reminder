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

fn default_theme_color() -> String {
    DEFAULT_THEME.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub work_minutes: u32,
    #[serde(default)]
    pub break_minutes: u32,
    #[serde(default)]
    pub custom_break_background: bool,
    pub break_background_path: Option<String>,
    #[serde(default = "default_custom_theme_color")]
    pub custom_theme_color: bool,
    #[serde(default = "default_theme_color")]
    pub theme_color: String,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default)]
    pub locale: AppLocale,
}

fn default_custom_theme_color() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            work_minutes: 30,
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

    let Ok(mut cfg) = serde_json::from_str::<AppConfig>(&raw) else {
        return AppConfig::default();
    };

    normalize_config(&mut cfg);
    cfg
}

pub fn load_or_create_config(app: &AppHandle) -> AppConfig {
    let path = match config_path(app) {
        Ok(p) => p,
        Err(_) => return AppConfig::default(),
    };

    let cfg = load_config(app);
    if !path.exists() {
        let _ = save_config_to_disk(app, &cfg);
    }
    cfg
}

fn normalize_config(cfg: &mut AppConfig) {
    if cfg.work_minutes == 0 {
        cfg.work_minutes = AppConfig::default().work_minutes;
    }
    if cfg.break_minutes == 0 {
        cfg.break_minutes = AppConfig::default().break_minutes;
    }
}

pub fn save_config_to_disk(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
