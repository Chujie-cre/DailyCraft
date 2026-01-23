use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tauri::{AppHandle, Emitter};

static IS_GENERATING: AtomicBool = AtomicBool::new(false);
static CURRENT_DIARY: Lazy<Mutex<DiaryState>> = Lazy::new(|| Mutex::new(DiaryState::default()));

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiaryState {
    pub is_generating: bool,
    pub content: String,
    pub error: String,
    pub date: String,
}

fn get_diary_dir() -> PathBuf {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("DailyCraft")
        .join("data")
        .join("diaries");
    fs::create_dir_all(&data_dir).ok();
    data_dir
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

async fn generate_diary_internal(
    activities_json: String,
    prompt: String,
    config: super::ai::AIConfig,
) -> Result<String, String> {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize)]
    struct ChatMessage {
        role: String,
        content: String,
    }
    
    #[derive(Serialize)]
    struct ChatRequest {
        model: String,
        messages: Vec<ChatMessage>,
    }
    
    #[derive(Deserialize)]
    struct ChatResponseMessage {
        content: String,
    }
    
    #[derive(Deserialize)]
    struct ChatChoice {
        message: ChatResponseMessage,
    }
    
    #[derive(Deserialize)]
    struct ChatResponse {
        choices: Vec<ChatChoice>,
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
    
    let chat_response: ChatResponse = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "AI未返回内容".to_string())
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
    let diary_dir = get_diary_dir();
    let file_path = diary_dir.join(format!("{}.md", date));
    
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
    let diary_dir = get_diary_dir();
    let mut diaries = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&diary_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".md") {
                    diaries.push(name.trim_end_matches(".md").to_string());
                }
            }
        }
    }
    
    diaries.sort_by(|a, b| b.cmp(a)); // 降序排列
    Ok(diaries)
}

#[tauri::command]
pub fn read_diary(date: String) -> Result<String, String> {
    let diary_dir = get_diary_dir();
    let file_path = diary_dir.join(format!("{}.md", date));
    
    fs::read_to_string(&file_path)
        .map_err(|e| format!("读取日记失败: {}", e))
}
