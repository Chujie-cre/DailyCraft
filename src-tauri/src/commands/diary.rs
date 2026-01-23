use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Emitter};
use crate::models::AppConfig;

static IS_GENERATING: AtomicBool = AtomicBool::new(false);
static CURRENT_DIARY: Lazy<Mutex<DiaryState>> = Lazy::new(|| Mutex::new(DiaryState::default()));

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiaryState {
    pub is_generating: bool,
    pub content: String,
    pub error: String,
    pub date: String,
}

fn get_diary_dir(date: &str) -> PathBuf {
    let config = AppConfig::load();
    let data_dir = config.data_dir.join(date);
    fs::create_dir_all(&data_dir).ok();
    data_dir
}

fn get_data_root() -> PathBuf {
    let config = AppConfig::load();
    config.data_dir
}

#[tauri::command]
pub fn get_diary_state() -> DiaryState {
    CURRENT_DIARY.lock().unwrap().clone()
}

#[tauri::command]
pub fn is_diary_generating() -> bool {
    IS_GENERATING.load(Ordering::SeqCst)
}

#[tauri::command]
pub async fn start_diary_generation(
    app: AppHandle,
    activities_json: String,
    prompt: String
) -> Result<(), String> {
    if IS_GENERATING.load(Ordering::SeqCst) {
        return Err("正在生成中，请稍候".to_string());
    }
    
    let config = super::ai::get_ai_config().await?;
    
    if config.api_key.is_empty() {
        return Err("请先配置API Key".to_string());
    }
    
    IS_GENERATING.store(true, Ordering::SeqCst);
    
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    // 更新状态
    {
        let mut state = CURRENT_DIARY.lock().unwrap();
        state.is_generating = true;
        state.content = String::new();
        state.error = String::new();
        state.date = date.clone();
    }
    
    // 在后台线程中执行流式生成
    let config_clone = config.clone();
    let date_clone = date.clone();
    tokio::spawn(async move {
        let result = generate_diary_stream(app.clone(), activities_json, prompt, config_clone).await;
        
        let mut state = CURRENT_DIARY.lock().unwrap();
        state.is_generating = false;
        IS_GENERATING.store(false, Ordering::SeqCst);
        
        match result {
            Ok(content) => {
                state.content = content.clone();
                state.error = String::new();
                // 自动保存到文件
                if let Err(e) = save_diary_to_file(&date_clone, &content) {
                    eprintln!("保存日记失败: {}", e);
                }
                let _ = app.emit("diary-complete", content);
            }
            Err(e) => {
                state.error = e.clone();
                let _ = app.emit("diary-error", e);
            }
        }
    });
    
    Ok(())
}

async fn generate_diary_stream(
    app: AppHandle,
    activities_json: String,
    prompt: String,
    config: super::ai::AIConfig,
) -> Result<String, String> {
    use futures_util::StreamExt;
    
    #[derive(serde::Serialize)]
    struct ChatMessage {
        role: String,
        content: String,
    }
    
    #[derive(serde::Serialize)]
    struct ChatRequest {
        model: String,
        messages: Vec<ChatMessage>,
        stream: bool,
    }
    
    #[derive(serde::Deserialize)]
    struct Delta {
        content: Option<String>,
    }
    
    #[derive(serde::Deserialize)]
    struct StreamChoice {
        delta: Delta,
        finish_reason: Option<String>,
    }
    
    #[derive(serde::Deserialize)]
    struct StreamChunk {
        choices: Vec<StreamChoice>,
    }
    
    let full_prompt = format!(
        "{}\n\n以下是今日的活动记录数据：\n{}",
        prompt,
        activities_json
    );
    
    let request = ChatRequest {
        model: config.model,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: full_prompt,
        }],
        stream: true,
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/chat/completions", config.base_url))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API请求失败: {}", error_text));
    }
    
    let mut full_content = String::new();
    let mut stream = response.bytes_stream();
    
    let mut buffer = String::new();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| format!("读取流失败: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);
        buffer.push_str(&text);
        
        // 处理可能跨chunk的数据
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();
            
            if line.is_empty() || line == "data: [DONE]" {
                continue;
            }
            
            if let Some(json_str) = line.strip_prefix("data: ") {
                if let Ok(chunk_data) = serde_json::from_str::<StreamChunk>(json_str) {
                    for choice in chunk_data.choices {
                        if let Some(content) = choice.delta.content {
                            if !content.is_empty() {
                                full_content.push_str(&content);
                                // 发送流式内容到前端
                                let _ = app.emit("diary-chunk", &content);
                                // 更新状态
                                if let Ok(mut state) = CURRENT_DIARY.lock() {
                                    state.content = full_content.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(full_content)
}

fn save_diary_to_file(date: &str, content: &str) -> Result<PathBuf, String> {
    let diary_dir = get_diary_dir(date);
    let file_path = diary_dir.join("diary.md");
    
    let markdown_content = format!(
        "# {} 日记\n\n{}\n\n---\n*由 DailyCraft AI 自动生成*\n",
        date, content
    );
    
    fs::write(&file_path, markdown_content)
        .map_err(|e| format!("保存文件失败: {}", e))?;
    
    Ok(file_path)
}

#[tauri::command]
pub fn save_diary(date: String, content: String) -> Result<String, String> {
    let path = save_diary_to_file(&date, &content)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_diary_list() -> Result<Vec<String>, String> {
    let data_root = get_data_root();
    let mut diaries = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&data_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            // 检查是否是日期目录（包含diary.md文件）
            if path.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    let diary_file = path.join("diary.md");
                    if diary_file.exists() {
                        diaries.push(name.to_string());
                    }
                }
            }
        }
    }
    
    diaries.sort_by(|a, b| b.cmp(a)); // 降序排列
    Ok(diaries)
}

#[tauri::command]
pub fn read_diary(date: String) -> Result<String, String> {
    let diary_dir = get_diary_dir(&date);
    let file_path = diary_dir.join("diary.md");
    
    fs::read_to_string(&file_path)
        .map_err(|e| format!("读取日记失败: {}", e))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_days: u32,
    pub today_events: u32,
    pub total_events: u32,
    pub today_diary: Option<String>,
}

#[tauri::command]
pub fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let data_root = get_data_root();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    
    let mut total_days = 0u32;
    let mut total_events = 0u32;
    let mut today_events = 0u32;
    let mut today_diary: Option<String> = None;
    
    // 遍历数据目录统计
    if let Ok(entries) = fs::read_dir(&data_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(date_str) = entry.file_name().to_str() {
                    // 检查是否有events.db文件
                    let db_path = path.join("events.db");
                    if db_path.exists() {
                        total_days += 1;
                        
                        // 统计该日期的事件数量
                        if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                            let result: Result<i64, _> = conn.query_row(
                                "SELECT COUNT(*) FROM events",
                                [],
                                |row| row.get(0)
                            );
                            if let Ok(count) = result {
                                total_events += count as u32;
                                if date_str == today {
                                    today_events = count as u32;
                                }
                            }
                        }
                    }
                    
                    // 检查今日日记
                    if date_str == today {
                        let diary_path = path.join("diary.md");
                        if diary_path.exists() {
                            if let Ok(content) = fs::read_to_string(&diary_path) {
                                // 提取日记摘要（去除markdown标题等）
                                let summary: String = content
                                    .lines()
                                    .filter(|l| !l.starts_with('#') && !l.starts_with('*') && !l.starts_with('-') && !l.trim().is_empty())
                                    .take(1)
                                    .collect::<Vec<_>>()
                                    .join(" ");
                                if !summary.is_empty() {
                                    let truncated = if summary.len() > 50 {
                                        format!("{}...", &summary[..50])
                                    } else {
                                        summary
                                    };
                                    today_diary = Some(truncated);
                                } else {
                                    today_diary = Some("已生成".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(DashboardStats {
        total_days,
        today_events,
        total_events,
        today_diary,
    })
}
