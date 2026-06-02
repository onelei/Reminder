use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn show_break_window(app: &AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("break") {
        let _ = w.show();
        let _ = w.set_focus();
        return Ok(());
    }

    WebviewWindowBuilder::new(app, "break", WebviewUrl::App("break".into()))
        .title("休息时间")
        .fullscreen(true)
        .decorations(false)
        .always_on_top(true)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn close_break_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("break") {
        let _ = w.close();
    }
}
