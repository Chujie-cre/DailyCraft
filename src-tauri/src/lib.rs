pub mod commands;
pub mod error;
pub mod models;
pub mod services;

use commands::system::*;
use commands::ai::*;
use commands::diary::*;
use commands::update::*;
use commands::deskpet::*;
use tauri::{
    Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--autostart"])))
        .setup(|app| {
            // 创建托盘菜单
            let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
            
            // 创建托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // 阻止窗口关闭，改为隐藏
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_active_window,
            record_app_focus,
            get_data_dir,
            set_data_dir,
            init_today_storage,
            get_today_event_count,
            get_total_event_count,
            record_keyboard_event,
            record_mouse_event,
            record_idle_event,
            get_today_events_grouped,
            get_events_grouped_by_date,
            get_today_events,
            get_icon_for_app,
            start_input_listening,
            get_input_stats,
            get_idle_seconds,
            is_input_listening,
            get_app_config,
            save_app_config,
            take_screenshot,
            take_screenshot_area,
            get_today_screenshots,
            get_screenshots_by_date,
            get_icon_by_app_name,
            open_folder,
            clear_cache,
            get_ai_config,
            save_ai_config,
            generate_diary,
            ai_chat,
            ai_chat_stream,
            save_chat_history,
            load_chat_history,
            save_notes,
            load_notes,
            get_diary_state,
            is_diary_generating,
            start_diary_generation,
            save_diary,
            get_diary_list,
            read_diary,
            get_dashboard_stats,
            ocr_image,
            get_ocr_data_by_date,
            save_ocr_record,
            check_for_update,
            set_update_preference,
            get_current_version,
            open_update_window,
            download_update,
            install_update,
            get_mouse_position,
            create_pet_window,
            close_pet_window,
            set_pet_ignore_cursor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
