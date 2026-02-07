use serde_json::json;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

/// 获取全局鼠标位置（因为桌宠窗口设置了鼠标穿透，JS无法获取鼠标事件）
#[tauri::command]
pub fn get_mouse_position() -> serde_json::Value {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        use windows::Win32::Foundation::POINT;
        
        let mut point = POINT { x: 0, y: 0 };
        unsafe {
            if GetCursorPos(&mut point).is_ok() {
                return json!({
                    "clientX": point.x,
                    "clientY": point.y
                });
            }
        }
    }
    
    json!(null)
}

/// 创建桌宠透明窗口
#[tauri::command]
pub async fn create_pet_window(app: tauri::AppHandle) -> Result<(), String> {
    // 如果窗口已存在，直接聚焦
    if let Some(window) = app.get_webview_window("deskpet") {
        let _ = window.set_focus();
        return Ok(());
    }

    // 获取主显示器尺寸
    let monitor = app.primary_monitor()
        .map_err(|e| format!("获取显示器信息失败: {}", e))?
        .ok_or("未找到主显示器")?;
    
    let size = monitor.size();
    let width = size.width as f64;
    let height = size.height as f64;
    let scale_factor = monitor.scale_factor();

    let window = WebviewWindowBuilder::new(
        &app,
        "deskpet",
        WebviewUrl::App("/deskpet".into()),
    )
    .title("DailyCraft Pet")
    .inner_size(width / scale_factor, height / scale_factor)
    .position(0.0, 0.0)
    .transparent(true)
    .shadow(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .decorations(false)
    .resizable(false)
    .build()
    .map_err(|e| format!("创建桌宠窗口失败: {}", e))?;

    // 设置鼠标穿透
    window.set_ignore_cursor_events(true)
        .map_err(|e| format!("设置鼠标穿透失败: {}", e))?;

    Ok(())
}

/// 关闭桌宠窗口
#[tauri::command]
pub async fn close_pet_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("deskpet") {
        window.close().map_err(|e| format!("关闭桌宠窗口失败: {}", e))?;
    }
    Ok(())
}

/// 设置桌宠窗口鼠标穿透状态
#[tauri::command]
pub async fn set_pet_ignore_cursor(app: tauri::AppHandle, ignore: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("deskpet") {
        window.set_ignore_cursor_events(ignore)
            .map_err(|e| format!("设置鼠标穿透失败: {}", e))?;
    }
    Ok(())
}
