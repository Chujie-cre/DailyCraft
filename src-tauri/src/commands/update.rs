use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use chrono::{Local, NaiveDate};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use futures_util::StreamExt;

use crate::models::AppConfig;

const GITHUB_REPO: &str = "Chujie-cre/DailyCraft";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub html_url: String,
    pub published_at: String,
    pub download_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCheckResult {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_info: Option<ReleaseInfo>,
    pub skipped_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdatePrefs {
    pub ignored_version: Option<String>,
    pub remind_after: Option<String>,
}

fn get_update_prefs_path() -> PathBuf {
    let config = AppConfig::load();
    config.data_dir.join("update_prefs.json")
}

fn load_update_prefs() -> UpdatePrefs {
    let path = get_update_prefs_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(prefs) = serde_json::from_str(&content) {
                return prefs;
            }
        }
    }
    UpdatePrefs::default()
}

fn save_update_prefs(prefs: &UpdatePrefs) -> Result<(), String> {
    let path = get_update_prefs_path();
    let content = serde_json::to_string_pretty(prefs)
        .map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    Ok(())
}

fn parse_version(version: &str) -> Vec<u32> {
    version
        .trim_start_matches('v')
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn compare_versions(current: &str, latest: &str) -> std::cmp::Ordering {
    let current_parts = parse_version(current);
    let latest_parts = parse_version(latest);
    
    for i in 0..std::cmp::max(current_parts.len(), latest_parts.len()) {
        let c = current_parts.get(i).unwrap_or(&0);
        let l = latest_parts.get(i).unwrap_or(&0);
        match c.cmp(l) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

#[tauri::command]
pub async fn check_for_update(force: bool) -> Result<UpdateCheckResult, String> {
    let prefs = load_update_prefs();
    
    // 获取最新Release
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "DailyCraft")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub API返回错误: {}", response.status()));
    }
    
    let release: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    let tag_name = release["tag_name"]
        .as_str()
        .ok_or("无法获取版本号")?
        .to_string();
    
    // 构建安装包下载地址
    let version_no_v = tag_name.trim_start_matches('v');
    let download_url = format!(
        "https://github.com/{}/releases/download/{}/dailycraft_{}_x64-setup.exe",
        GITHUB_REPO, tag_name, version_no_v
    );
    
    let release_info = ReleaseInfo {
        tag_name: tag_name.clone(),
        name: release["name"].as_str().unwrap_or(&tag_name).to_string(),
        body: release["body"].as_str().unwrap_or("").to_string(),
        html_url: release["html_url"].as_str().unwrap_or("").to_string(),
        published_at: release["published_at"].as_str().unwrap_or("").to_string(),
        download_url,
    };
    
    let latest_version = tag_name.trim_start_matches('v');
    
    // 比较版本
    if compare_versions(CURRENT_VERSION, latest_version) != std::cmp::Ordering::Less {
        return Ok(UpdateCheckResult {
            has_update: false,
            current_version: CURRENT_VERSION.to_string(),
            latest_version: Some(latest_version.to_string()),
            release_info: None,
            skipped_reason: Some("已是最新版本".to_string()),
        });
    }
    
    // 非强制检查时，检查用户偏好
    if !force {
        // 检查是否忽略此版本
        if let Some(ignored) = &prefs.ignored_version {
            if ignored == latest_version {
                return Ok(UpdateCheckResult {
                    has_update: false,
                    current_version: CURRENT_VERSION.to_string(),
                    latest_version: Some(latest_version.to_string()),
                    release_info: None,
                    skipped_reason: Some("用户已忽略此版本".to_string()),
                });
            }
        }
        
        // 检查是否在提醒日期之前
        if let Some(remind_after) = &prefs.remind_after {
            if let Ok(remind_date) = NaiveDate::parse_from_str(remind_after, "%Y-%m-%d") {
                let today = Local::now().date_naive();
                if today < remind_date {
                    return Ok(UpdateCheckResult {
                        has_update: false,
                        current_version: CURRENT_VERSION.to_string(),
                        latest_version: Some(latest_version.to_string()),
                        release_info: None,
                        skipped_reason: Some(format!("用户选择{}后提醒", remind_after)),
                    });
                }
            }
        }
    }
    
    Ok(UpdateCheckResult {
        has_update: true,
        current_version: CURRENT_VERSION.to_string(),
        latest_version: Some(latest_version.to_string()),
        release_info: Some(release_info),
        skipped_reason: None,
    })
}

#[tauri::command]
pub fn set_update_preference(action: String, version: Option<String>) -> Result<(), String> {
    let mut prefs = load_update_prefs();
    
    match action.as_str() {
        "ignore" => {
            prefs.ignored_version = version;
            prefs.remind_after = None;
        }
        "remind_tomorrow" => {
            let tomorrow = Local::now().date_naive() + chrono::Duration::days(1);
            prefs.remind_after = Some(tomorrow.format("%Y-%m-%d").to_string());
            prefs.ignored_version = None;
        }
        "clear" => {
            prefs.ignored_version = None;
            prefs.remind_after = None;
        }
        _ => return Err("未知操作".to_string()),
    }
    
    save_update_prefs(&prefs)
}

#[tauri::command]
pub fn get_current_version() -> String {
    CURRENT_VERSION.to_string()
}

#[tauri::command]
pub async fn open_update_window(
    app: AppHandle,
    current_version: String,
    latest_version: String,
    release_info: ReleaseInfo
) -> Result<(), String> {
    // 检查窗口是否已存在
    if let Some(window) = app.get_webview_window("update") {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }
    
    // 序列化release_info为URL参数
    let release_json = serde_json::to_string(&release_info).map_err(|e| e.to_string())?;
    let encoded = urlencoding::encode(&release_json);
    
    let url = format!(
        "/update?current={}&latest={}&release={}",
        urlencoding::encode(&current_version),
        urlencoding::encode(&latest_version),
        encoded
    );
    
    WebviewWindowBuilder::new(&app, "update", WebviewUrl::App(url.into()))
        .title("DailyCraft - 应用更新")
        .inner_size(580.0, 720.0)   // 设置新窗口尺寸   宽*高
        .resizable(false)
        .center()
        .decorations(true)
        .build()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[derive(Clone, Serialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f64,
}

#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    download_url: String,
    version: String
) -> Result<String, String> {
    let config = AppConfig::load();
    let download_dir = config.data_dir.join("downloads");
    fs::create_dir_all(&download_dir).map_err(|e| format!("创建下载目录失败: {}", e))?;
    
    let filename = format!("dailycraft_{}_x64-setup.exe", version);
    let file_path = download_dir.join(&filename);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&download_url)
        .header("User-Agent", "DailyCraft")
        .send()
        .await
        .map_err(|e| format!("下载请求失败: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("下载失败: HTTP {}", response.status()));
    }
    
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    
    let mut file = File::create(&file_path)
        .map_err(|e| format!("创建文件失败: {}", e))?;
    
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("下载数据失败: {}", e))?;
        file.write_all(&chunk).map_err(|e| format!("写入文件失败: {}", e))?;
        
        downloaded += chunk.len() as u64;
        let percentage = if total_size > 0 {
            (downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };
        
        let _ = app.emit("download-progress", DownloadProgress {
            downloaded,
            total: total_size,
            percentage,
        });
    }
    
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn install_update(file_path: String) -> Result<(), String> {
    Command::new(&file_path)
        .spawn()
        .map_err(|e| format!("启动安装程序失败: {}", e))?;
    
    std::process::exit(0);
}
