use crate::models::{AppConfig, RawEvent, EventType};
use crate::services::{StorageService, WindowTracker, get_app_icon, input_tracker, ScreenshotService};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::PathBuf;
use std::fs;

/// 全局配置状态
static CONFIG: Mutex<Option<AppConfig>> = Mutex::new(None);

fn get_config() -> AppConfig {
    let guard = CONFIG.lock().unwrap();
    guard.clone().unwrap_or_else(|| AppConfig::load())
}

fn set_config(config: AppConfig) {
    // 保存到文件
    let _ = config.save();
    // 更新内存缓存
    let mut guard = CONFIG.lock().unwrap();
    *guard = Some(config);
}

#[derive(Serialize)]
pub struct ActiveWindowInfo {
    pub app_name: String,
    pub window_title: String,
    pub exe_path: String,
}

/// 前端展示用的事件数据
#[derive(Serialize)]
pub struct EventForDisplay {
    pub id: String,
    pub timestamp: String,
    pub event_type: String,
    pub app: Option<String>,
    pub window_title: Option<String>,
    pub exe_path: Option<String>,
    pub key_count: Option<u32>,
    pub mouse_distance: Option<f64>,
    pub click_count: Option<u32>,
    pub time_display: String,  // 格式化的时间显示 如 "09:12"
}

/// 按类型分组的事件数据（供Flow画布使用）
#[derive(Serialize)]
pub struct GroupedEvents {
    pub app_focus: Vec<EventForDisplay>,
    pub keyboard: Vec<EventForDisplay>,
    pub mouse: Vec<EventForDisplay>,
    pub idle: Vec<EventForDisplay>,
}

/// 获取当前活动窗口信息
#[tauri::command]
pub fn get_active_window() -> std::result::Result<ActiveWindowInfo, String> {
    let tracker = WindowTracker::new();
    match tracker.get_active_window() {
        Ok(info) => Ok(ActiveWindowInfo {
            app_name: info.app_name,
            window_title: info.window_title,
            exe_path: info.exe_path,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// 获取应用图标（base64编码的PNG）
#[tauri::command]
pub fn get_icon_for_app(exe_path: String) -> Option<String> {
    get_app_icon(&exe_path)
}

/// 记录应用焦点事件
#[tauri::command]
pub fn record_app_focus(app: String, window_title: String, exe_path: String) -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let event = RawEvent::app_focus(app, window_title, exe_path);
    
    storage.append_raw_event(&event).map_err(|e| e.to_string())
}

/// 获取数据存储目录
#[tauri::command]
pub fn get_data_dir() -> String {
    let config = get_config();
    let storage = StorageService::new(config);
    storage.get_data_dir()
}

/// 初始化当日存储
#[tauri::command]
pub fn init_today_storage() -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    storage.init_today().map_err(|e| e.to_string())
}

/// 获取当日原始事件数量
#[tauri::command]
pub fn get_today_event_count() -> std::result::Result<usize, String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let events = storage.read_raw_events().map_err(|e| e.to_string())?;
    Ok(events.len())
}

/// 设置数据存储目录
#[tauri::command]
pub fn set_data_dir(path: String) -> std::result::Result<(), String> {
    let mut config = get_config();
    config.data_dir = PathBuf::from(path);
    set_config(config);
    Ok(())
}

/// 记录键盘事件（关联当前应用）
#[tauri::command]
pub fn record_keyboard_event(key_count: u32, app: String, window_title: String, exe_path: String) -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let event = RawEvent::keyboard(key_count, app, window_title, exe_path);
    storage.append_raw_event(&event).map_err(|e| e.to_string())
}

/// 记录鼠标事件（关联当前应用）
#[tauri::command]
pub fn record_mouse_event(distance: f64, click_count: u32, app: String, window_title: String, exe_path: String) -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let event = RawEvent::mouse(distance, click_count, app, window_title, exe_path);
    storage.append_raw_event(&event).map_err(|e| e.to_string())
}

/// 记录空闲事件
#[tauri::command]
pub fn record_idle_event(duration_sec: u64) -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let event = RawEvent::idle(duration_sec);
    storage.append_raw_event(&event).map_err(|e| e.to_string())
}

/// 获取当日所有事件（按类型分组，供Flow画布使用）
#[tauri::command]
pub fn get_today_events_grouped() -> std::result::Result<GroupedEvents, String> {
    get_events_grouped_by_date(chrono::Local::now().format("%Y-%m-%d").to_string())
}

/// 获取指定日期的事件（按类型分组）
#[tauri::command]
pub fn get_events_grouped_by_date(date: String) -> std::result::Result<GroupedEvents, String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let events = storage.read_raw_events_by_date(&date).map_err(|e| e.to_string())?;
    
    let mut grouped = GroupedEvents {
        app_focus: Vec::new(),
        keyboard: Vec::new(),
        mouse: Vec::new(),
        idle: Vec::new(),
    };
    
    for (idx, event) in events.iter().enumerate() {
        let display = EventForDisplay {
            id: format!("event_{}", idx),
            timestamp: event.timestamp.to_rfc3339(),
            event_type: match event.event_type {
                EventType::AppFocus => "app_focus".to_string(),
                EventType::Keyboard => "keyboard".to_string(),
                EventType::Mouse => "mouse".to_string(),
                EventType::Idle => "idle".to_string(),
            },
            app: event.app.clone(),
            window_title: event.window_title.clone(),
            exe_path: event.exe_path.clone(),
            key_count: event.metadata.key_count,
            mouse_distance: event.metadata.mouse_distance,
            click_count: event.metadata.click_count,
            time_display: event.timestamp.format("%H:%M:%S").to_string(),
        };
        
        match event.event_type {
            EventType::AppFocus => grouped.app_focus.push(display),
            EventType::Keyboard => grouped.keyboard.push(display),
            EventType::Mouse => grouped.mouse.push(display),
            EventType::Idle => grouped.idle.push(display),
        }
    }
    
    Ok(grouped)
}

/// 获取当日所有事件（平铺列表）
#[tauri::command]
pub fn get_today_events() -> std::result::Result<Vec<EventForDisplay>, String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let events = storage.read_raw_events().map_err(|e| e.to_string())?;
    
    let displays: Vec<EventForDisplay> = events
        .iter()
        .enumerate()
        .map(|(idx, event)| EventForDisplay {
            id: format!("event_{}", idx),
            timestamp: event.timestamp.to_rfc3339(),
            event_type: match event.event_type {
                EventType::AppFocus => "app_focus".to_string(),
                EventType::Keyboard => "keyboard".to_string(),
                EventType::Mouse => "mouse".to_string(),
                EventType::Idle => "idle".to_string(),
            },
            app: event.app.clone(),
            window_title: event.window_title.clone(),
            exe_path: event.exe_path.clone(),
            key_count: event.metadata.key_count,
            mouse_distance: event.metadata.mouse_distance,
            click_count: event.metadata.click_count,
            time_display: event.timestamp.format("%H:%M:%S").to_string(),
        })
        .collect();
    
    Ok(displays)
}

/// 输入统计数据结构
#[derive(Serialize)]
pub struct InputStatsResponse {
    pub key_count: u32,
    pub click_count: u32,
    pub mouse_distance: f64,
    pub idle_seconds: u64,
}

/// 启动全局输入监听
#[tauri::command]
pub fn start_input_listening() -> std::result::Result<(), String> {
    input_tracker::start_listening();
    Ok(())
}

/// 获取输入统计并重置计数器
#[tauri::command]
pub fn get_input_stats() -> InputStatsResponse {
    let stats = input_tracker::get_and_reset_stats();
    InputStatsResponse {
        key_count: stats.key_count,
        click_count: stats.click_count,
        mouse_distance: stats.mouse_distance,
        idle_seconds: stats.idle_seconds,
    }
}

/// 获取当前空闲时间（秒）
#[tauri::command]
pub fn get_idle_seconds() -> u64 {
    input_tracker::get_idle_seconds()
}

/// 检查输入监听是否运行中
#[tauri::command]
pub fn is_input_listening() -> bool {
    input_tracker::is_listening()
}

/// 应用配置响应结构
#[derive(Serialize, Deserialize)]
pub struct AppConfigResponse {
    pub poll_interval_ms: u64,
    pub idle_threshold_sec: u64,
    pub screenshot_enabled: bool,
    pub screenshot_trigger_sec: u32,
    pub screenshot_interval_sec: u32,
    pub screenshot_mode: String,
    pub screenshot_hotkey: String,
}

/// 获取应用配置
#[tauri::command]
pub fn get_app_config() -> AppConfigResponse {
    let config = get_config();
    AppConfigResponse {
        poll_interval_ms: config.poll_interval_ms,
        idle_threshold_sec: config.idle_threshold_sec,
        screenshot_enabled: config.screenshot_enabled,
        screenshot_trigger_sec: config.screenshot_trigger_sec,
        screenshot_interval_sec: config.screenshot_interval_sec,
        screenshot_mode: config.screenshot_mode,
        screenshot_hotkey: config.screenshot_hotkey,
    }
}

/// 保存应用配置
#[tauri::command]
pub fn save_app_config(
    poll_interval_ms: u64,
    idle_threshold_sec: u64,
    screenshot_enabled: bool,
    screenshot_trigger_sec: u32,
    screenshot_interval_sec: u32,
    screenshot_mode: String,
    screenshot_hotkey: String,
) -> std::result::Result<(), String> {
    let mut config = get_config();
    config.poll_interval_ms = poll_interval_ms;
    config.idle_threshold_sec = idle_threshold_sec;
    config.screenshot_enabled = screenshot_enabled;
    config.screenshot_trigger_sec = screenshot_trigger_sec;
    config.screenshot_interval_sec = screenshot_interval_sec;
    config.screenshot_mode = screenshot_mode;
    config.screenshot_hotkey = screenshot_hotkey;
    set_config(config);
    Ok(())
}

/// 截图响应
#[derive(Serialize)]
pub struct ScreenshotResponse {
    pub success: bool,
    pub filepath: Option<String>,
    pub error: Option<String>,
}

/// 手动截图 - 根据配置选择全屏或应用窗口模式，截图后在后台异步执行OCR
#[tauri::command]
pub async fn take_screenshot(app_name: String) -> ScreenshotResponse {
    let config = get_config();
    let screenshot_dir = config.get_screenshots_dir();
    let service = ScreenshotService::new(screenshot_dir);
    
    // 根据截图模式选择截图方式
    let capture_result = if config.screenshot_mode == "app_window" {
        // 应用窗口模式：获取当前窗口位置并截取
        let tracker = crate::services::WindowTracker::new();
        match tracker.get_active_window_rect() {
            Ok(rect) => {
                if rect.width > 0 && rect.height > 0 {
                    service.capture_area(&app_name, rect.x, rect.y, rect.width, rect.height)
                } else {
                    service.capture_full_screen(&app_name)
                }
            }
            Err(_) => service.capture_full_screen(&app_name)
        }
    } else {
        // 全屏模式
        service.capture_full_screen(&app_name)
    };
    
    match capture_result {
        Ok(path) => {
            let filepath = path.to_string_lossy().to_string();
            
            // 在后台异步执行OCR，不阻塞主线程
            let filepath_clone = filepath.clone();
            let app_name_clone = app_name.clone();
            tokio::spawn(async move {
                let date = chrono::Local::now().format("%Y-%m-%d").to_string();
                let timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                
                match crate::services::extract_text_from_image(&filepath_clone).await {
                    Ok(text) => {
                        if !text.trim().is_empty() {
                            let record = OcrRecord {
                                timestamp,
                                image_path: filepath_clone.clone(),
                                text,
                                app_name: Some(app_name_clone),
                            };
                            let _ = save_ocr_to_db(&date, &record);
                        }
                    }
                    Err(_) => {}
                }
            });
            
            ScreenshotResponse {
                success: true,
                filepath: Some(filepath),
                error: None,
            }
        },
        Err(e) => ScreenshotResponse {
            success: false,
            filepath: None,
            error: Some(e),
        },
    }
}

/// 截取指定区域
#[tauri::command]
pub fn take_screenshot_area(app_name: String, x: i32, y: i32, width: u32, height: u32) -> ScreenshotResponse {
    let config = get_config();
    let screenshot_dir = config.get_screenshots_dir();
    let service = ScreenshotService::new(screenshot_dir);
    
    match service.capture_area(&app_name, x, y, width, height) {
        Ok(path) => ScreenshotResponse {
            success: true,
            filepath: Some(path.to_string_lossy().to_string()),
            error: None,
        },
        Err(e) => ScreenshotResponse {
            success: false,
            filepath: None,
            error: Some(e),
        },
    }
}

/// 获取今日截图列表
#[tauri::command]
pub fn get_today_screenshots() -> Vec<String> {
    let config = get_config();
    let screenshot_dir = config.get_screenshots_dir();
    let service = ScreenshotService::new(screenshot_dir);
    
    service.get_today_screenshots()
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}

/// 根据应用名称获取图标（从最近的事件中查找exe路径）
#[tauri::command]
pub fn get_icon_by_app_name(app_name: String) -> Option<String> {
    let config = get_config();
    let storage = StorageService::new(config.clone());
    
    // 从今日事件中查找该应用的exe路径
    if let Ok(events) = storage.read_raw_events() {
        for event in events.iter().rev() {
            if let Some(ref app) = event.app {
                if app == &app_name {
                    if let Some(ref exe_path) = event.exe_path {
                        return get_app_icon(exe_path);
                    }
                }
            }
        }
    }
    
    None
}

/// 清理缓存（删除所有日期的截图文件）
#[tauri::command]
pub fn clear_cache() -> std::result::Result<(), String> {
    let config = get_config();
    let data_dir = &config.data_dir;
    
    if !data_dir.exists() {
        return Ok(());
    }
    
    // 遍历所有日期目录，删除其中的screenshots子目录
    let entries = fs::read_dir(data_dir)
        .map_err(|e| format!("读取数据目录失败: {}", e))?;
    
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let screenshots_dir = path.join("screenshots");
            if screenshots_dir.exists() && screenshots_dir.is_dir() {
                fs::remove_dir_all(&screenshots_dir)
                    .map_err(|e| format!("删除截图目录失败: {}", e))?;
            }
        }
    }
    
    Ok(())
}

/// 打开文件夹
#[tauri::command]
pub fn open_folder(path: String) -> std::result::Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开文件夹失败: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开文件夹失败: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开文件夹失败: {}", e))?;
    }
    Ok(())
}

/// 获取指定日期的截图列表
#[tauri::command]
pub fn get_screenshots_by_date(date: String) -> Vec<String> {
    let config = get_config();
    let data_dir = config.data_dir.join(&date).join("screenshots");
    
    if !data_dir.exists() {
        return vec![];
    }
    
    fs::read_dir(&data_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.extension()
                        .map(|ext| ext == "jpg" || ext == "png")
                        .unwrap_or(false)
                })
                .map(|p| p.to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// OCR识别图片文本
#[tauri::command]
pub async fn ocr_image(image_path: String) -> Result<String, String> {
    crate::services::extract_text_from_image(&image_path).await
}

/// 获取指定日期的OCR数据
#[tauri::command]
pub fn get_ocr_data_by_date(date: String) -> Result<Vec<OcrRecord>, String> {
    let config = get_config();
    let db_path = config.data_dir.join(&date).join("events.db");
    
    if !db_path.exists() {
        return Ok(vec![]);
    }
    
    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;
    
    // 确保ocr_records表存在
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ocr_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            image_path TEXT NOT NULL,
            text TEXT NOT NULL,
            app_name TEXT
        )",
        [],
    ).map_err(|e| format!("创建表失败: {}", e))?;
    
    let mut stmt = conn.prepare(
        "SELECT timestamp, image_path, text, app_name FROM ocr_records ORDER BY timestamp"
    ).map_err(|e| format!("准备查询失败: {}", e))?;
    
    let records = stmt.query_map([], |row| {
        Ok(OcrRecord {
            timestamp: row.get(0)?,
            image_path: row.get(1)?,
            text: row.get(2)?,
            app_name: row.get(3)?,
        })
    }).map_err(|e| format!("查询失败: {}", e))?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(records)
}

/// 保存OCR记录
#[tauri::command]
pub fn save_ocr_record(date: String, record: OcrRecord) -> Result<(), String> {
    let config = get_config();
    let data_dir = config.data_dir.join(&date);
    
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("创建目录失败: {}", e))?;
    
    let db_path = data_dir.join("events.db");
    
    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;
    
    // 确保ocr_records表存在
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ocr_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            image_path TEXT NOT NULL,
            text TEXT NOT NULL,
            app_name TEXT
        )",
        [],
    ).map_err(|e| format!("创建表失败: {}", e))?;
    
    conn.execute(
        "INSERT INTO ocr_records (timestamp, image_path, text, app_name) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![record.timestamp, record.image_path, record.text, record.app_name],
    ).map_err(|e| format!("插入记录失败: {}", e))?;
    
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OcrRecord {
    pub timestamp: String,
    pub image_path: String,
    pub text: String,
    pub app_name: Option<String>,
}

/// 内部函数：保存OCR记录到数据库
fn save_ocr_to_db(date: &str, record: &OcrRecord) -> Result<(), String> {
    let config = get_config();
    let data_dir = config.data_dir.join(date);
    
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("创建目录失败: {}", e))?;
    
    let db_path = data_dir.join("events.db");
    
    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ocr_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            image_path TEXT NOT NULL,
            text TEXT NOT NULL,
            app_name TEXT
        )",
        [],
    ).map_err(|e| format!("创建表失败: {}", e))?;
    
    conn.execute(
        "INSERT INTO ocr_records (timestamp, image_path, text, app_name) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![record.timestamp, record.image_path, record.text, record.app_name],
    ).map_err(|e| format!("插入记录失败: {}", e))?;
    
    Ok(())
}
