use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::models::AppConfig;

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
    let config = AppConfig::load();
    fs::create_dir_all(&config.data_dir).ok();
    config.data_dir.join("ai_config.json")
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
