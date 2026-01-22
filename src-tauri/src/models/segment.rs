use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 活动强度级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActivityLevel {
    High,
    Medium,
    Low,
    Idle,
}

/// 输入密度统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputDensity {
    pub keyboard: u32,
    pub mouse: u32,
}

/// 行为状态段
/// 将高频RawEvent合并后的聚合数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSegment {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub app: String,
    pub window_title: String,
    pub duration_min: u32,
    pub activity_level: ActivityLevel,
    pub input_density: InputDensity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_path: Option<String>,
}

impl StateSegment {
    /// 创建新的状态段
    pub fn new(app: String, window_title: String, start_time: DateTime<Utc>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            start_time,
            end_time: start_time,
            app,
            window_title,
            duration_min: 0,
            activity_level: ActivityLevel::Low,
            input_density: InputDensity::default(),
            screenshot_path: None,
        }
    }

    /// 更新结束时间和持续时长
    pub fn update_end_time(&mut self, end_time: DateTime<Utc>) {
        self.end_time = end_time;
        let duration = (end_time - self.start_time).num_minutes();
        self.duration_min = duration.max(0) as u32;
    }

    /// 累加输入统计
    pub fn add_input(&mut self, keyboard: u32, mouse: u32) {
        self.input_density.keyboard += keyboard;
        self.input_density.mouse += mouse;
        self.update_activity_level();
    }

    /// 根据输入密度更新活动级别
    fn update_activity_level(&mut self) {
        let total = self.input_density.keyboard + self.input_density.mouse;
        let duration = self.duration_min.max(1);
        let density = total / duration;

        self.activity_level = match density {
            d if d >= 50 => ActivityLevel::High,
            d if d >= 20 => ActivityLevel::Medium,
            d if d >= 1 => ActivityLevel::Low,
            _ => ActivityLevel::Idle,
        };
    }

    /// 设置截图路径
    pub fn set_screenshot(&mut self, path: String) {
        self.screenshot_path = Some(path);
    }
}

/// 每日数据封包（供AI使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummaryPack {
    pub date: String,
    pub segments: Vec<StateSegment>,
    pub total_active_minutes: u32,
    pub total_idle_minutes: u32,
    pub app_usage: Vec<AppUsage>,
}

/// 应用使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsage {
    pub app: String,
    pub total_minutes: u32,
    pub percentage: f32,
}
