use std::path::Path;
use std::process::{Command, Stdio, Child, ChildStdin, ChildStdout};
use std::sync::Mutex;
use std::io::{BufRead, BufReader, Write};
use once_cell::sync::Lazy;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// OCR常驻进程
struct OcrProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

static OCR_PROCESS: Lazy<Mutex<Option<OcrProcess>>> = Lazy::new(|| Mutex::new(None));

/// 获取或启动OCR进程
fn get_or_start_ocr_process() -> Result<(), String> {
    let mut guard = OCR_PROCESS.lock().map_err(|_| "OCR锁获取失败")?;
    
    // 检查进程是否存活
    if let Some(ref mut proc) = *guard {
        match proc.child.try_wait() {
            Ok(None) => return Ok(()), // 进程仍在运行
            _ => {} // 进程已退出，需要重启
        }
    }
    
    // 启动新进程
    let script_path = get_ocr_script_path()?;
    
    let mut cmd = Command::new("python");
    cmd.arg(&script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .env("PYTHONIOENCODING", "utf-8");
    
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    let mut child = cmd.spawn()
        .map_err(|e| format!("启动OCR进程失败: {}", e))?;
    
    let stdin = child.stdin.take()
        .ok_or("无法获取OCR进程stdin")?;
    let stdout = child.stdout.take()
        .ok_or("无法获取OCR进程stdout")?;
    
    let mut reader = BufReader::new(stdout);
    
    // 等待就绪信号
    let mut ready_line = String::new();
    reader.read_line(&mut ready_line)
        .map_err(|e| format!("读取OCR就绪信号失败: {}", e))?;
    
    let ready_json: serde_json::Value = serde_json::from_str(&ready_line)
        .map_err(|e| format!("解析OCR就绪信号失败: {}", e))?;
    
    if let Some(error) = ready_json.get("error").and_then(|v| v.as_str()) {
        return Err(error.to_string());
    }
    
    *guard = Some(OcrProcess { child, stdin, stdout: reader });
    Ok(())
}

fn get_ocr_script_path() -> Result<std::path::PathBuf, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取程序路径失败: {}", e))?
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    
    let script_path = exe_dir.join("scripts").join("ocr_service.py");
    
    if script_path.exists() {
        return Ok(script_path);
    }
    
    let dev_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts").join("ocr_service.py");
    if dev_path.exists() {
        return Ok(dev_path);
    }
    
    Err("OCR脚本不存在".to_string())
}

/// 使用RapidOCR进行文字识别（常驻进程模式）
pub async fn extract_text_from_image(image_path: &str) -> Result<String, String> {
    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("图片文件不存在: {}", image_path));
    }
    
    // 确保OCR进程运行
    get_or_start_ocr_process()?;
    
    // 发送请求并获取结果
    let mut guard = OCR_PROCESS.lock().map_err(|_| "OCR锁获取失败")?;
    let proc = guard.as_mut().ok_or("OCR进程未启动")?;
    
    // 发送请求
    let request = serde_json::json!({"image_path": image_path});
    writeln!(proc.stdin, "{}", request)
        .map_err(|e| format!("发送OCR请求失败: {}", e))?;
    proc.stdin.flush()
        .map_err(|e| format!("刷新OCR请求失败: {}", e))?;
    
    // 读取响应
    let mut response_line = String::new();
    proc.stdout.read_line(&mut response_line)
        .map_err(|e| format!("读取OCR响应失败: {}", e))?;
    
    let response: serde_json::Value = serde_json::from_str(&response_line)
        .map_err(|e| format!("解析OCR响应失败: {}", e))?;
    
    if let Some(error) = response.get("error").and_then(|v| v.as_str()) {
        return Err(error.to_string());
    }
    
    response.get("text")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or("OCR响应中没有text字段".to_string())
}
