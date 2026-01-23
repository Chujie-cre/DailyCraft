use once_cell::sync::Lazy;
use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

/// 全局输入统计
static KEY_COUNT: AtomicU32 = AtomicU32::new(0);
static CLICK_COUNT: AtomicU32 = AtomicU32::new(0);
static MOUSE_DISTANCE: Lazy<Mutex<f64>> = Lazy::new(|| Mutex::new(0.0));
static LAST_MOUSE_POS: Lazy<Mutex<Option<(f64, f64)>>> = Lazy::new(|| Mutex::new(None));
static LAST_INPUT_TIME: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));
static IS_LISTENING: AtomicBool = AtomicBool::new(false);

/// 输入统计数据
#[derive(Debug, Clone)]
pub struct InputStats {
    pub key_count: u32,
    pub click_count: u32,
    pub mouse_distance: f64,
    pub idle_seconds: u64,
}

/// 获取并重置输入统计
pub fn get_and_reset_stats() -> InputStats {
    let key_count = KEY_COUNT.swap(0, Ordering::SeqCst);
    let click_count = CLICK_COUNT.swap(0, Ordering::SeqCst);
    
    let mouse_distance = {
        let mut dist = MOUSE_DISTANCE.lock().unwrap();
        let val = *dist;
        *dist = 0.0;
        val
    };
    
    let idle_seconds = {
        let last_input = LAST_INPUT_TIME.lock().unwrap();
        last_input.elapsed().as_secs()
    };
    
    InputStats {
        key_count,
        click_count,
        mouse_distance,
        idle_seconds,
    }
}

/// 获取当前空闲时间（秒）
pub fn get_idle_seconds() -> u64 {
    let last_input = LAST_INPUT_TIME.lock().unwrap();
    last_input.elapsed().as_secs()
}

/// 更新最后输入时间
fn update_last_input_time() {
    let mut last_input = LAST_INPUT_TIME.lock().unwrap();
    *last_input = Instant::now();
}

/// 事件回调处理
fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(_) => {
            KEY_COUNT.fetch_add(1, Ordering::SeqCst);
            update_last_input_time();
        }
        EventType::ButtonPress(_) => {
            CLICK_COUNT.fetch_add(1, Ordering::SeqCst);
            update_last_input_time();
        }
        EventType::MouseMove { x, y } => {
            let mut last_pos = LAST_MOUSE_POS.lock().unwrap();
            if let Some((last_x, last_y)) = *last_pos {
                let dx = x - last_x;
                let dy = y - last_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                let mut total_dist = MOUSE_DISTANCE.lock().unwrap();
                *total_dist += distance;
            }
            *last_pos = Some((x, y));
            update_last_input_time();
        }
        EventType::Wheel { .. } => {
            update_last_input_time();
        }
        _ => {}
    }
}

/// 启动全局输入监听（在单独线程中运行）
pub fn start_listening() {
    if IS_LISTENING.swap(true, Ordering::SeqCst) {
        return; // 已经在监听
    }
    
    thread::spawn(|| {
        if let Err(e) = listen(callback) {
            eprintln!("输入监听错误: {:?}", e);
            IS_LISTENING.store(false, Ordering::SeqCst);
        }
    });
}

/// 检查是否正在监听
pub fn is_listening() -> bool {
    IS_LISTENING.load(Ordering::SeqCst)
}
