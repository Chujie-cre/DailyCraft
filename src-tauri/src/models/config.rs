use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 数据存储根目录
    pub data_dir: PathBuf,
    /// 截图黑名单应用
    pub screenshot_blacklist: Vec<String>,
    /// 空闲判定阈值（秒）
    pub idle_threshold_sec: u64,
    /// 事件采集间隔（毫秒）
    pub poll_interval_ms: u64,
    /// 是否启用截图
    pub screenshot_enabled: bool,
    /// 长时间稳定状态截图阈值（分钟）
    pub stable_screenshot_threshold_min: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("DailyCraft"),
            screenshot_blacklist: vec![
                "1Password".to_string(),
                "Bitwarden".to_string(),
                "KeePass".to_string(),
            ],
            idle_threshold_sec: 300, // 5分钟
            poll_interval_ms: 1000,  // 1秒
            screenshot_enabled: true,
            stable_screenshot_threshold_min: 20,
        }
    }
}

impl AppConfig {
    /// 获取当日数据目录
    pub fn get_today_dir(&self) -> PathBuf {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.data_dir.join(&today)
    }

    /// 获取原始事件日志路径
    pub fn get_raw_events_path(&self) -> PathBuf {
        self.get_today_dir().join("raw_events.jsonl")
    }

    /// 获取状态段文件路径
    pub fn get_segments_path(&self) -> PathBuf {
        self.get_today_dir().join("state_segments.json")
    }

    /// 获取截图目录
    pub fn get_screenshots_dir(&self) -> PathBuf {
        self.get_today_dir().join("screenshots")
    }

    /// 检查应用是否在截图黑名单中
    pub fn is_screenshot_blacklisted(&self, app: &str) -> bool {
        self.screenshot_blacklist
            .iter()
            .any(|b| app.to_lowercase().contains(&b.to_lowercase()))
    }
}
