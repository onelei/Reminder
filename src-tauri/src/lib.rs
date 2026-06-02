mod config;
mod timer;
mod windows;

use config::{is_english, load_config, save_config_to_disk, AppConfig, SharedConfig};
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, State,
};
use tauri_plugin_notification::NotificationExt;
use timer::{
    create_timer, end_work, skip_break, snapshot, spawn_timer_loop, start_break, start_work,
    SharedTimer, TimerSnapshot,
};

struct AppState {
    config: SharedConfig,
    timer: SharedTimer,
}

#[tauri::command]
fn get_config(state: State<'_, AppState>) -> AppConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
fn save_config(app: AppHandle, state: State<'_, AppState>, config: AppConfig) -> Result<(), String> {
    {
        let mut current = state.config.lock().unwrap();
        *current = config.clone();
    }
    save_config_to_disk(&app, &config)?;
    sync_autostart(&app, config.autostart)?;
    let snap = snapshot(&state.timer, &state.config);
    let _ = app.emit("timer-tick", &snap);
    Ok(())
}

#[tauri::command]
fn get_timer_state(state: State<'_, AppState>) -> TimerSnapshot {
    snapshot(&state.timer, &state.config)
}

#[tauri::command]
fn start_work_cmd(app: AppHandle, state: State<'_, AppState>) -> Result<TimerSnapshot, String> {
    let snap = start_work(&state.timer, &state.config)?;
    let _ = app.emit("timer-tick", &snap);
    Ok(snap)
}

#[tauri::command]
fn end_work_cmd(app: AppHandle, state: State<'_, AppState>) -> TimerSnapshot {
    let snap = end_work(&state.timer, &state.config);
    let _ = app.emit("timer-tick", &snap);
    snap
}

#[tauri::command]
fn start_break_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    long: bool,
) -> Result<TimerSnapshot, String> {
    let snap = start_break(&state.timer, &state.config, long)?;
    windows::show_break_window(&app)?;
    let _ = app.emit("timer-tick", &snap);
    Ok(snap)
}

#[tauri::command]
fn skip_break_cmd(app: AppHandle, state: State<'_, AppState>) -> Result<TimerSnapshot, String> {
    let snap = skip_break(&state.timer, &state.config)?;
    windows::close_break_window(&app);
    windows::close_break_prompt(&app);
    let _ = app.emit("timer-tick", &snap);
    Ok(snap)
}

#[tauri::command]
fn show_main_window(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

#[tauri::command]
fn hide_main_window(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

#[tauri::command]
fn close_break_prompt_cmd(app: AppHandle) {
    windows::close_break_prompt(&app);
}

#[tauri::command]
async fn pick_break_background(app: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file = app
        .dialog()
        .file()
        .add_filter("图片", &["png", "jpg", "jpeg", "webp", "gif"])
        .blocking_pick_file();

    Ok(file.map(|p| p.to_string()))
}

fn sync_autostart(app: &AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let autostart = app.autolaunch();
    if enabled {
        autostart.enable().map_err(|e| e.to_string())
    } else {
        autostart.disable().map_err(|e| e.to_string())
    }
}

fn handle_work_complete(app: &AppHandle, state: &AppState, use_long_break: bool) {
    let cfg = state.config.lock().unwrap().clone();

    if cfg.auto_break_mode {
        if start_break(&state.timer, &state.config, use_long_break).is_ok() {
            let _ = windows::show_break_window(app);
            let snap = snapshot(&state.timer, &state.config);
            let _ = app.emit("timer-tick", &snap);
        }
        return;
    }

    let _ = windows::show_break_prompt(app, use_long_break);

    let (title, body) = if is_english(cfg.locale) {
        ("Time for a break", "Nice work! How about a short break?")
    } else {
        ("该休息啦~", "辛苦了！休息一下吧？")
    };
    let _ = app.notification().builder().title(title).body(body).show();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let loaded = load_config(app.handle());
            let config: SharedConfig = std::sync::Arc::new(Mutex::new(loaded.clone()));
            let timer = create_timer();
            let state = AppState {
                config: config.clone(),
                timer: timer.clone(),
            };
            app.manage(state);

            sync_autostart(app.handle(), loaded.autostart).ok();
            spawn_timer_loop(app.handle().clone(), timer, config.clone());

            let tray_en = is_english(loaded.locale);
            let show_label = if tray_en { "Show main window" } else { "显示主窗口" };
            let quit_label = if tray_en { "Quit" } else { "退出" };
            let show_item = MenuItem::with_id(app, "show", show_label, true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", quit_label, true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let icon = app.default_window_icon().cloned().unwrap();
            TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Reminder")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            if let Some(main_window) = app.get_webview_window("main") {
                let win = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win.hide();
                    }
                });
            }

            use tauri::Listener;

            let handle = app.handle().clone();
            let event_handle = handle.clone();
            handle.listen("timer-work-complete", move |event| {
                let payload = event.payload();
                let use_long = serde_json::from_str::<serde_json::Value>(payload)
                    .ok()
                    .and_then(|v| v.get("useLongBreak").and_then(|b| b.as_bool()))
                    .unwrap_or(false);

                if let Some(state) = event_handle.try_state::<AppState>() {
                    handle_work_complete(&event_handle, &state, use_long);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_timer_state,
            start_work_cmd,
            end_work_cmd,
            start_break_cmd,
            skip_break_cmd,
            show_main_window,
            hide_main_window,
            close_break_prompt_cmd,
            pick_break_background,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
