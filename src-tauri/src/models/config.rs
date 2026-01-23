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
    /// 是否启用自动截图
    pub screenshot_enabled: bool,
    /// 应用停留多少秒后自动截图
    pub screenshot_trigger_sec: u32,
    /// 自动截图间隔（秒）
    pub screenshot_interval_sec: u32,
    /// 截图模式：app_window=应用窗口, full_screen=全屏
    pub screenshot_mode: String,
    /// 手动截图快捷键
    pub screenshot_hotkey: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("DailyCraft")
                .join("data"),
            screenshot_blacklist: vec![
                "1Password".to_string(),
                "Bitwarden".to_string(),
                "KeePass".to_string(),
            ],
            idle_threshold_sec: 300,        // 5分钟
            poll_interval_ms: 1000,         // 1秒
            screenshot_enabled: false,      // 默认关闭
            screenshot_trigger_sec: 30,     // 停留30秒后截图
            screenshot_interval_sec: 60,    // 每60秒截图一次
            screenshot_mode: "full_screen".to_string(),
            screenshot_hotkey: "Alt+]".to_string(),
        }
    }
}

impl AppConfig {
    /// 获取配置文件路径
    pub fn get_config_file_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("DailyCraft")
            .join("config.json")
    }

    /// 从文件加载配置
    pub fn load() -> Self {
        let config_path = Self::get_config_file_path();
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_path = Self::get_config_file_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&config_path, content)
    }

    /// 获取当日数据目录
    pub fn get_today_dir(&self) -> PathBuf {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.data_dir.join(&today)
    }
    
    /// 获取指定日期的数据目录
    pub fn get_date_dir(&self, date: &str) -> PathBuf {
        self.data_dir.join(date)
    }

    /// 获取原始事件日志路径
    pub fn get_raw_events_path(&self) -> PathBuf {
        self.get_today_dir().join("raw_events.jsonl")
    }
    
    /// 获取指定日期的原始事件日志路径
    pub fn get_raw_events_path_by_date(&self, date: &str) -> PathBuf {
        self.get_date_dir(date).join("raw_events.jsonl")
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
