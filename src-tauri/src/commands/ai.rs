use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::models::AppConfig;

/// 获取基础数据目录（data_dir的父目录，用于存放配置文件）
fn get_base_data_dir() -> PathBuf {
    let config = AppConfig::load();
    // data_dir是data/data，我们需要data
    config.data_dir.parent().unwrap_or(&config.data_dir).to_path_buf()
}

fn get_chat_history_path() -> PathBuf {
    let base_dir = get_base_data_dir();
    fs::create_dir_all(&base_dir).ok();
    base_dir.join("chat_history.json")
}

#[tauri::command]
pub fn save_chat_history(sessions: String) -> Result<(), String> {
    let path = get_chat_history_path();
    fs::write(&path, sessions).map_err(|e| format!("保存对话历史失败: {}", e))
}

#[tauri::command]
pub fn load_chat_history() -> Result<String, String> {
    let path = get_chat_history_path();
    if path.exists() {
        fs::read_to_string(&path).map_err(|e| format!("加载对话历史失败: {}", e))
    } else {
        Ok("[]".to_string())
    }
}

fn get_notes_path() -> PathBuf {
    let base_dir = get_base_data_dir();
    fs::create_dir_all(&base_dir).ok();
    base_dir.join("notes.json")
}

#[tauri::command]
pub fn save_notes(notes: String) -> Result<(), String> {
    let path = get_notes_path();
    fs::write(&path, notes).map_err(|e| format!("保存笔记失败: {}", e))
}

#[tauri::command]
pub fn load_notes() -> Result<String, String> {
    let path = get_notes_path();
    if path.exists() {
        fs::read_to_string(&path).map_err(|e| format!("加载笔记失败: {}", e))
    } else {
        Ok("[]".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "qwen-plus".to_string(),
            base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

fn get_ai_config_path() -> PathBuf {
    let base_dir = get_base_data_dir();
    fs::create_dir_all(&base_dir).ok();
    base_dir.join("ai_config.json")
}

#[tauri::command]
pub async fn get_ai_config() -> Result<AIConfig, String> {
    let config_path = get_ai_config_path();
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("读取配置失败: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("解析配置失败: {}", e))
    } else {
        Ok(AIConfig::default())
    }
}

#[tauri::command]
pub async fn save_ai_config(config: AIConfig) -> Result<(), String> {
    let config_path = get_ai_config_path();
    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    fs::write(&config_path, content)
        .map_err(|e| format!("保存配置失败: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn generate_diary(activities_json: String, prompt: String) -> Result<String, String> {
    let config = get_ai_config().await?;
    
    if config.api_key.is_empty() {
        return Err("请先配置API Key".to_string());
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

#[tauri::command]
pub async fn ai_chat_stream(
    app: tauri::AppHandle,
    system_prompt: String,
    user_message: String
) -> Result<(), String> {
    use futures_util::StreamExt;
    use tauri::Emitter;
    
    let config = get_ai_config().await?;
    
    if config.api_key.is_empty() {
        return Err("请先配置API Key".to_string());
    }
    
    #[derive(serde::Serialize)]
    struct StreamChatRequest {
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
    }
    
    #[derive(serde::Deserialize)]
    struct StreamChunk {
        choices: Vec<StreamChoice>,
    }
    
    let request = StreamChatRequest {
        model: config.model,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_message,
            },
        ],
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
        let _ = app.emit("chat-error", error_text.clone());
        return Err(format!("API请求失败: {}", error_text));
    }
    
    let mut stream = response.bytes_stream();
    
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                let text = String::from_utf8_lossy(&chunk);
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            break;
                        }
                        if let Ok(chunk_data) = serde_json::from_str::<StreamChunk>(data) {
                            if let Some(choice) = chunk_data.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    let _ = app.emit("chat-chunk", content.clone());
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let _ = app.emit("chat-error", e.to_string());
                return Err(e.to_string());
            }
        }
    }
    
    let _ = app.emit("chat-complete", ());
    Ok(())
}

#[tauri::command]
pub async fn ai_chat(system_prompt: String, user_message: String) -> Result<String, String> {
    let config = get_ai_config().await?;
    
    if config.api_key.is_empty() {
        return Err("请先配置API Key".to_string());
    }
    
    let request = ChatRequest {
        model: config.model,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_message,
            },
        ],
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
