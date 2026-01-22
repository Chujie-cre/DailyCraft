use std::collections::HashMap;
use std::sync::Mutex;

/// 图标缓存
static ICON_CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

fn get_cache() -> std::sync::MutexGuard<'static, Option<HashMap<String, String>>> {
    ICON_CACHE.lock().unwrap()
}

/// 获取应用图标（base64编码的PNG）
#[cfg(target_os = "windows")]
pub fn get_app_icon(exe_path: &str) -> Option<String> {
    if exe_path.is_empty() {
        return None;
    }
    
    // 检查缓存
    {
        let cache = get_cache();
        if let Some(ref map) = *cache {
            if let Some(icon) = map.get(exe_path) {
                return Some(icon.clone());
            }
        }
    }
    
    // 提取图标
    let icon_data = extract_icon_windows(exe_path)?;
    
    // 存入缓存
    {
        let mut cache = get_cache();
        if cache.is_none() {
            *cache = Some(HashMap::new());
        }
        if let Some(ref mut map) = *cache {
            map.insert(exe_path.to_string(), icon_data.clone());
        }
    }
    
    Some(icon_data)
}

#[cfg(target_os = "windows")]
fn extract_icon_windows(exe_path: &str) -> Option<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::UI::Shell::ExtractIconExW;
    use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};
    use windows::Win32::Graphics::Gdi::{
        GetDIBits, CreateCompatibleDC, DeleteDC, SelectObject, DeleteObject,
        BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, GetObjectW, BITMAP,
    };
    use windows::Win32::Foundation::HWND;
    
    unsafe {
        // 转换路径为宽字符
        let wide_path: Vec<u16> = OsStr::new(exe_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        // 提取图标
        let mut large_icon: HICON = HICON::default();
        let count = ExtractIconExW(
            windows::core::PCWSTR(wide_path.as_ptr()),
            0,
            Some(&mut large_icon),
            None,
            1,
        );
        
        if count == 0 || large_icon.is_invalid() {
            return None;
        }
        
        // 获取图标信息
        let mut icon_info = ICONINFO::default();
        if GetIconInfo(large_icon, &mut icon_info).is_err() {
            DestroyIcon(large_icon).ok();
            return None;
        }
        
        // 获取位图信息
        let mut bmp = BITMAP::default();
        if GetObjectW(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bmp as *mut _ as *mut _),
        ) == 0 {
            DestroyIcon(large_icon).ok();
            DeleteObject(icon_info.hbmColor).ok();
            DeleteObject(icon_info.hbmMask).ok();
            return None;
        }
        
        let width = bmp.bmWidth as u32;
        let height = bmp.bmHeight as u32;
        
        // 创建DC
        let hdc = CreateCompatibleDC(None);
        if hdc.is_invalid() {
            DestroyIcon(large_icon).ok();
            DeleteObject(icon_info.hbmColor).ok();
            DeleteObject(icon_info.hbmMask).ok();
            return None;
        }
        
        // 准备位图信息
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // 自上而下
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [Default::default()],
        };
        
        // 分配像素缓冲区
        let pixel_count = (width * height) as usize;
        let mut pixels: Vec<u8> = vec![0; pixel_count * 4];
        
        let old_bmp = SelectObject(hdc, icon_info.hbmColor);
        
        GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            height,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );
        
        SelectObject(hdc, old_bmp);
        DeleteDC(hdc).ok();
        DestroyIcon(large_icon).ok();
        DeleteObject(icon_info.hbmColor).ok();
        DeleteObject(icon_info.hbmMask).ok();
        
        // BGRA -> RGBA
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2);
        }
        
        // 创建PNG图像
        let img = image::RgbaImage::from_raw(width, height, pixels)?;
        
        let mut png_data: Vec<u8> = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_data);
        img.write_to(&mut cursor, image::ImageFormat::Png).ok()?;
        
        // Base64编码
        let base64_str = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &png_data,
        );
        
        Some(format!("data:image/png;base64,{}", base64_str))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_app_icon(_exe_path: &str) -> Option<String> {
    None
}
