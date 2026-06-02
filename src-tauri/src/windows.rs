use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn show_break_prompt(app: &AppHandle, use_long: bool) -> Result<(), String> {
    if app.get_webview_window("break-prompt").is_some() {
        if let Some(w) = app.get_webview_window("break-prompt") {
            let _ = w.show();
            let _ = w.set_focus();
        }
        return Ok(());
    }

    let path = if use_long {
        "break-prompt?long=true"
    } else {
        "break-prompt"
    };

    let mut builder = WebviewWindowBuilder::new(
        app,
        "break-prompt",
        WebviewUrl::App(path.into()),
    )
    .title("休息提醒")
    .inner_size(340.0, 200.0)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true);

    if let Ok(monitor) = app.primary_monitor() {
        if let Some(monitor) = monitor {
            let size = monitor.size();
            let scale = monitor.scale_factor();
            let win_w = 340.0 * scale;
            let win_h = 200.0 * scale;
            let x = size.width as f64 - win_w - 16.0 * scale;
            let y = size.height as f64 - win_h - 64.0 * scale;
            builder = builder.position(x, y);
        }
    }

    builder.build().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn close_break_prompt(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("break-prompt") {
        let _ = w.close();
    }
}

pub fn show_break_window(app: &AppHandle) -> Result<(), String> {
    close_break_prompt(app);

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
