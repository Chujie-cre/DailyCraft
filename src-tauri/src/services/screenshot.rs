use std::path::PathBuf;
use std::fs;
use std::io::Cursor;
use chrono::Local;

#[cfg(windows)]
use screenshots::Screen;

/// 截图服务
pub struct ScreenshotService {
    screenshots_dir: PathBuf,
}

impl ScreenshotService {
    pub fn new(screenshots_dir: PathBuf) -> Self {
        Self { screenshots_dir }
    }

    /// 确保截图目录存在
    fn ensure_dir(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.screenshots_dir)
    }

    /// 生成截图文件名
    fn generate_filename(&self, app_name: &str) -> PathBuf {
        let timestamp = Local::now().format("%H-%M-%S").to_string();
        let safe_app_name: String = app_name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .take(30)
            .collect();
        self.screenshots_dir.join(format!("{}_{}.jpg", timestamp, safe_app_name))
    }

    /// 压缩并保存图片为JPEG格式（质量70%）
    #[cfg(windows)]
    fn save_as_jpeg(&self, width: u32, height: u32, rgba_data: Vec<u8>, filepath: &PathBuf) -> Result<(), String> {
        use image::{ImageBuffer, Rgba, codecs::jpeg::JpegEncoder};
        
        // 从原始数据创建ImageBuffer
        let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = 
            ImageBuffer::from_raw(width, height, rgba_data)
                .ok_or("创建图像缓冲区失败")?;
        
        // 转换为RGB（JPEG不支持透明度）
        let rgb_img = image::DynamicImage::ImageRgba8(img_buffer).to_rgb8();
        
        // 使用JPEG编码器，质量设为70
        let mut buffer = Cursor::new(Vec::new());
        let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 70);
        encoder.encode_image(&rgb_img)
            .map_err(|e| format!("JPEG编码失败: {}", e))?;
        
        fs::write(filepath, buffer.into_inner())
            .map_err(|e| format!("保存文件失败: {}", e))?;
        
        Ok(())
    }

    /// 截取全屏
    #[cfg(windows)]
    pub fn capture_full_screen(&self, app_name: &str) -> Result<PathBuf, String> {
        self.ensure_dir().map_err(|e| format!("创建截图目录失败: {}", e))?;
        
        let screens = Screen::all().map_err(|e| format!("获取屏幕失败: {}", e))?;
        if screens.is_empty() {
            return Err("未找到屏幕".to_string());
        }
        
        // 获取主屏幕
        let screen = &screens[0];
        let capture = screen.capture().map_err(|e| format!("截图失败: {}", e))?;
        
        let filepath = self.generate_filename(app_name);
        // 直接保存为JPEG压缩格式
        let (w, h) = capture.dimensions();
        self.save_as_jpeg(w, h, capture.into_raw(), &filepath)?;
        
        Ok(filepath)
    }

    #[cfg(not(windows))]
    pub fn capture_full_screen(&self, app_name: &str) -> Result<PathBuf, String> {
        Err("截图功能仅支持Windows".to_string())
    }

    /// 截取指定区域（用于应用窗口截图）
    #[cfg(windows)]
    pub fn capture_area(&self, app_name: &str, x: i32, y: i32, width: u32, height: u32) -> Result<PathBuf, String> {
        self.ensure_dir().map_err(|e| format!("创建截图目录失败: {}", e))?;
        
        let screens = Screen::all().map_err(|e| format!("获取屏幕失败: {}", e))?;
        if screens.is_empty() {
            return Err("未找到屏幕".to_string());
        }
        
        let screen = &screens[0];
        let capture = screen.capture_area(x, y, width, height)
            .map_err(|e| format!("截图失败: {}", e))?;
        
        let filepath = self.generate_filename(app_name);
        // 直接保存为JPEG压缩格式
        let (w, h) = capture.dimensions();
        self.save_as_jpeg(w, h, capture.into_raw(), &filepath)?;
        
        Ok(filepath)
    }

    #[cfg(not(windows))]
    pub fn capture_area(&self, _app_name: &str, _x: i32, _y: i32, _width: u32, _height: u32) -> Result<PathBuf, String> {
        Err("截图功能仅支持Windows".to_string())
    }

    /// 获取今日截图列表
    pub fn get_today_screenshots(&self) -> Vec<PathBuf> {
        if !self.screenshots_dir.exists() {
            return vec![];
        }
        
        fs::read_dir(&self.screenshots_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().map(|ext| ext == "jpg" || ext == "png").unwrap_or(false))
                    .collect()
            })
            .unwrap_or_default()
    }
}
