use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

/// 事件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AppFocus,
    Keyboard,
    Mouse,
    Idle,
}

/// 事件元数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_duration_sec: Option<u64>,
}

/// 原始事件数据结构
/// 用于记录最底层的行为事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEvent {
    pub timestamp: DateTime<Local>,
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exe_path: Option<String>,
    #[serde(default)]
    pub metadata: EventMetadata,
}

impl RawEvent {
    /// 创建应用焦点事件
    pub fn app_focus(app: String, window_title: String, exe_path: String) -> Self {
        Self {
            timestamp: Local::now(),
            event_type: EventType::AppFocus,
            app: Some(app),
            window_title: Some(window_title),
            exe_path: if exe_path.is_empty() { None } else { Some(exe_path) },
            metadata: EventMetadata::default(),
        }
    }

    /// 创建键盘事件
    pub fn keyboard(key_count: u32) -> Self {
        Self {
            timestamp: Local::now(),
            event_type: EventType::Keyboard,
            app: None,
            window_title: None,
            exe_path: None,
            metadata: EventMetadata {
                key_count: Some(key_count),
                ..Default::default()
            },
        }
    }

    /// 创建鼠标事件
    pub fn mouse(distance: f64, click_count: u32) -> Self {
        Self {
            timestamp: Local::now(),
            event_type: EventType::Mouse,
            app: None,
            window_title: None,
            exe_path: None,
            metadata: EventMetadata {
                mouse_distance: Some(distance),
                click_count: Some(click_count),
                ..Default::default()
            },
        }
    }

    /// 创建空闲事件
    pub fn idle(duration_sec: u64) -> Self {
        Self {
            timestamp: Local::now(),
            event_type: EventType::Idle,
            app: None,
            window_title: None,
            exe_path: None,
            metadata: EventMetadata {
                idle_duration_sec: Some(duration_sec),
                ..Default::default()
            },
        }
    }
}
