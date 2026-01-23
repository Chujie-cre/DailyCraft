use crate::error::{AppError, Result};

/// 窗口信息
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub app_name: String,
    pub window_title: String,
    pub process_id: u32,
    pub exe_path: String,
}

/// 窗口矩形区域
#[derive(Debug, Clone)]
pub struct WindowRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// 窗口追踪服务
pub struct WindowTracker;

impl WindowTracker {
    pub fn new() -> Self {
        Self
    }

    /// 获取当前活动窗口信息
    #[cfg(target_os = "windows")]
    pub fn get_active_window(&self) -> Result<WindowInfo> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
        };
        use windows::Win32::System::Threading::{
            OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT,
            PROCESS_QUERY_LIMITED_INFORMATION,
        };

        unsafe {
            // 获取前台窗口句柄
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.0 == std::ptr::null_mut() {
                return Err(AppError::WindowTracker("No active window".to_string()));
            }

            // 获取窗口标题
            let mut title_buf: [u16; 512] = [0; 512];
            let title_len = GetWindowTextW(hwnd, &mut title_buf);
            let window_title = if title_len > 0 {
                OsString::from_wide(&title_buf[..title_len as usize])
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };

            // 获取进程ID
            let mut process_id: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut process_id));

            // 获取进程名称和路径
            let (app_name, exe_path) = if process_id > 0 {
                if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                    let mut name_buf: [u16; 512] = [0; 512];
                    let mut name_len: u32 = 512;
                    if QueryFullProcessImageNameW(
                        handle,
                        PROCESS_NAME_FORMAT(0),
                        windows::core::PWSTR(name_buf.as_mut_ptr()),
                        &mut name_len,
                    ).is_ok() {
                        let full_path = OsString::from_wide(&name_buf[..name_len as usize])
                            .to_string_lossy()
                            .to_string();
                        // 提取文件名
                        let name = std::path::Path::new(&full_path)
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| full_path.clone());
                        (name, full_path)
                    } else {
                        ("Unknown".to_string(), String::new())
                    }
                } else {
                    ("Unknown".to_string(), String::new())
                }
            } else {
                ("Unknown".to_string(), String::new())
            };

            Ok(WindowInfo {
                app_name,
                window_title,
                process_id,
                exe_path,
            })
        }
    }

    /// macOS/Linux 占位实现
    #[cfg(not(target_os = "windows"))]
    pub fn get_active_window(&self) -> Result<WindowInfo> {
        Err(AppError::WindowTracker(
            "Window tracking not implemented for this platform".to_string(),
        ))
    }

    /// 获取当前活动窗口的位置和大小
    #[cfg(target_os = "windows")]
    pub fn get_active_window_rect(&self) -> Result<WindowRect> {
        use windows::Win32::Foundation::{HWND, RECT};
        use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
        use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.0 == std::ptr::null_mut() {
                return Err(AppError::WindowTracker("No active window".to_string()));
            }

            let mut rect: RECT = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut rect).is_ok() {
                let width = (rect.right - rect.left).max(0) as u32;
                let height = (rect.bottom - rect.top).max(0) as u32;
                Ok(WindowRect {
                    x: rect.left,
                    y: rect.top,
                    width,
                    height,
                })
            } else {
                Err(AppError::WindowTracker("Failed to get window rect".to_string()))
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_active_window_rect(&self) -> Result<WindowRect> {
        Err(AppError::WindowTracker(
            "Window tracking not implemented for this platform".to_string(),
        ))
    }
}

impl Default for WindowTracker {
    fn default() -> Self {
        Self::new()
    }
}
