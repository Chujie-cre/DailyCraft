use crate::models::{AppConfig, RawEvent, EventType};
use crate::services::{StorageService, WindowTracker, get_app_icon};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::PathBuf;
use std::collections::HashMap;

/// 全局配置状态
static CONFIG: Mutex<Option<AppConfig>> = Mutex::new(None);

fn get_config() -> AppConfig {
    let guard = CONFIG.lock().unwrap();
    guard.clone().unwrap_or_default()
}

fn set_config(config: AppConfig) {
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

/// 记录键盘事件
#[tauri::command]
pub fn record_keyboard_event(key_count: u32) -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let event = RawEvent::keyboard(key_count);
    storage.append_raw_event(&event).map_err(|e| e.to_string())
}

/// 记录鼠标事件
#[tauri::command]
pub fn record_mouse_event(distance: f64, click_count: u32) -> std::result::Result<(), String> {
    let config = get_config();
    let storage = StorageService::new(config);
    let event = RawEvent::mouse(distance, click_count);
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
    let config = get_config();
    let storage = StorageService::new(config);
    let events = storage.read_raw_events().map_err(|e| e.to_string())?;
    
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
