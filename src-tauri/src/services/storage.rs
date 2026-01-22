use crate::error::{AppError, Result};
use crate::models::{AppConfig, RawEvent, StateSegment};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// 存储服务
pub struct StorageService {
    config: AppConfig,
}

impl StorageService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// 确保目录存在
    fn ensure_dir(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// 初始化当日存储目录
    pub fn init_today(&self) -> Result<()> {
        let today_dir = self.config.get_today_dir();
        self.ensure_dir(&today_dir)?;
        self.ensure_dir(&self.config.get_screenshots_dir())?;
        Ok(())
    }

    /// 追加写入原始事件（JSONL格式）
    pub fn append_raw_event(&self, event: &RawEvent) -> Result<()> {
        self.init_today()?;
        
        let path = self.config.get_raw_events_path();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        let json = serde_json::to_string(event)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }

    /// 批量写入原始事件
    pub fn append_raw_events(&self, events: &[RawEvent]) -> Result<()> {
        if events.is_empty() {
            return Ok(());
        }

        self.init_today()?;
        
        let path = self.config.get_raw_events_path();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

        for event in events {
            let json = serde_json::to_string(event)?;
            writeln!(file, "{}", json)?;
        }
        Ok(())
    }

    /// 读取当日所有原始事件
    pub fn read_raw_events(&self) -> Result<Vec<RawEvent>> {
        let path = self.config.get_raw_events_path();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut events = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let event: RawEvent = serde_json::from_str(&line)?;
                events.push(event);
            }
        }

        Ok(events)
    }

    /// 保存状态段列表
    pub fn save_segments(&self, segments: &[StateSegment]) -> Result<()> {
        self.init_today()?;
        
        let path = self.config.get_segments_path();
        let json = serde_json::to_string_pretty(segments)?;
        fs::write(&path, json)?;
        Ok(())
    }

    /// 读取状态段列表
    pub fn read_segments(&self) -> Result<Vec<StateSegment>> {
        let path = self.config.get_segments_path();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)?;
        let segments: Vec<StateSegment> = serde_json::from_str(&content)?;
        Ok(segments)
    }

    /// 获取截图保存路径
    pub fn get_screenshot_path(&self, app: &str) -> Result<String> {
        self.init_today()?;
        
        let timestamp = chrono::Local::now().format("%H%M%S").to_string();
        let safe_app = app.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        let filename = format!("{}_{}.png", timestamp, safe_app);
        let path = self.config.get_screenshots_dir().join(filename);
        
        Ok(path.to_string_lossy().to_string())
    }

    /// 获取数据目录路径
    pub fn get_data_dir(&self) -> String {
        self.config.data_dir.to_string_lossy().to_string()
    }
}
