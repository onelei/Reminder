use tauri::{
    AppHandle, Manager, Monitor, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder,
};

fn is_break_label(label: &str) -> bool {
    label == "break" || label.starts_with("break-")
}

fn break_window_labels(app: &AppHandle) -> Vec<String> {
    app.webview_windows()
        .into_keys()
        .filter(|label| is_break_label(label))
        .collect()
}

fn create_break_window_on_monitor(
    app: &AppHandle,
    label: &str,
    monitor: &Monitor,
) -> Result<(), String> {
    let pos = monitor.position();
    let size = monitor.size();
    let scale = monitor.scale_factor();

    let logical_x = pos.x as f64 / scale;
    let logical_y = pos.y as f64 / scale;
    let logical_w = size.width as f64 / scale;
    let logical_h = size.height as f64 / scale;

    let window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("break".into()))
        .title("休息时间")
        .decorations(false)
        .always_on_top(true)
        .resizable(false)
        .visible(false)
        .position(logical_x, logical_y)
        .inner_size(logical_w, logical_h)
        .build()
        .map_err(|e| e.to_string())?;

    window
        .set_position(PhysicalPosition::new(pos.x, pos.y))
        .map_err(|e| e.to_string())?;
    window
        .set_size(PhysicalSize::new(size.width, size.height))
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn show_break_window(app: &AppHandle) -> Result<(), String> {
    let existing = break_window_labels(app);
    if !existing.is_empty() {
        for (i, label) in existing.iter().enumerate() {
            if let Some(w) = app.get_webview_window(label) {
                let _ = w.show();
                if i == 0 {
                    let _ = w.set_focus();
                }
            }
        }
        return Ok(());
    }

    let monitors = app.available_monitors().map_err(|e| e.to_string())?;
    if monitors.is_empty() {
        WebviewWindowBuilder::new(app, "break-0", WebviewUrl::App("break".into()))
            .title("休息时间")
            .fullscreen(true)
            .decorations(false)
            .always_on_top(true)
            .build()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    for (i, monitor) in monitors.iter().enumerate() {
        let label = format!("break-{i}");
        create_break_window_on_monitor(app, &label, monitor)?;
    }

    Ok(())
}

pub fn close_break_window(app: &AppHandle) {
    for label in break_window_labels(app) {
        if let Some(w) = app.get_webview_window(&label) {
            let _ = w.close();
        }
    }
}
