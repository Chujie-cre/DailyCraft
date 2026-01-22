pub mod commands;
pub mod error;
pub mod models;
pub mod services;

use commands::system::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_active_window,
            record_app_focus,
            get_data_dir,
            set_data_dir,
            init_today_storage,
            get_today_event_count,
            record_keyboard_event,
            record_mouse_event,
            record_idle_event,
            get_today_events_grouped,
            get_today_events,
            get_icon_for_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
