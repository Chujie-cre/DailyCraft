import { invoke } from '@tauri-apps/api/core';

export interface EventForDisplay {
  id: string;
  timestamp: string;
  event_type: 'app_focus' | 'keyboard' | 'mouse' | 'idle';
  app?: string;
  window_title?: string;
  exe_path?: string;
  key_count?: number;
  mouse_distance?: number;
  click_count?: number;
  time_display: string;
}

export interface GroupedEvents {
  app_focus: EventForDisplay[];
  keyboard: EventForDisplay[];
  mouse: EventForDisplay[];
  idle: EventForDisplay[];
}

export interface ActiveWindowInfo {
  app_name: string;
  window_title: string;
  exe_path: string;
}

export interface InputStats {
  key_count: number;
  click_count: number;
  mouse_distance: number;
  idle_seconds: number;
}

export interface AppConfig {
  poll_interval_ms: number;
  idle_threshold_sec: number;
  screenshot_enabled: boolean;
  screenshot_trigger_sec: number;
  screenshot_interval_sec: number;
  screenshot_mode: string;
  screenshot_hotkey: string;
}

export interface ScreenshotResponse {
  success: boolean;
  filepath: string | null;
  error: string | null;
}

export interface DashboardStats {
  total_days: number;
  today_events: number;
  total_events: number;
  today_diary: string | null;
}

export interface OcrRecord {
  timestamp: string;
  image_path: string;
  text: string;
  app_name: string | null;
}

export const activityApi = {
  getGroupedEvents: () => invoke<GroupedEvents>('get_today_events_grouped'),
  
  // OCR相关
  ocrImage: (imagePath: string) => invoke<string>('ocr_image', { imagePath }),
  
  getOcrDataByDate: (date: string) => invoke<OcrRecord[]>('get_ocr_data_by_date', { date }),
  
  saveOcrRecord: (date: string, record: OcrRecord) => invoke('save_ocr_record', { date, record }),
  
  getGroupedEventsByDate: (date: string) => invoke<GroupedEvents>('get_events_grouped_by_date', { date }),
  
  getAllEvents: () => invoke<EventForDisplay[]>('get_today_events'),
  
  getActiveWindow: () => invoke<ActiveWindowInfo>('get_active_window'),
  
  recordAppFocus: (app: string, windowTitle: string, exePath: string) => 
    invoke('record_app_focus', { app, windowTitle, exePath }),
  
  recordKeyboard: (keyCount: number, app: string, windowTitle: string, exePath: string) => 
    invoke('record_keyboard_event', { keyCount, app, windowTitle, exePath }),
  
  recordMouse: (distance: number, clickCount: number, app: string, windowTitle: string, exePath: string) => 
    invoke('record_mouse_event', { distance, clickCount, app, windowTitle, exePath }),
  
  recordIdle: (durationSec: number) => 
    invoke('record_idle_event', { durationSec }),
  
  setDataDir: (path: string) => invoke('set_data_dir', { path }),
  
  getDataDir: () => invoke<string>('get_data_dir'),
  
  initTodayStorage: () => invoke('init_today_storage'),
  
  getTodayEventCount: () => invoke<number>('get_today_event_count'),
  
  getAppIcon: (exePath: string) => invoke<string | null>('get_icon_for_app', { exePath }),
  
  getIconForApp: (appName: string) => invoke<string | null>('get_icon_by_app_name', { appName }),
  
  // 输入监听相关
  startInputListening: () => invoke('start_input_listening'),
  
  getInputStats: () => invoke<InputStats>('get_input_stats'),
  
  getIdleSeconds: () => invoke<number>('get_idle_seconds'),
  
  isInputListening: () => invoke<boolean>('is_input_listening'),
  
  // 应用配置相关
  getAppConfig: () => invoke<AppConfig>('get_app_config'),
  
  saveAppConfig: (config: AppConfig) => invoke('save_app_config', {
    pollIntervalMs: config.poll_interval_ms,
    idleThresholdSec: config.idle_threshold_sec,
    screenshotEnabled: config.screenshot_enabled,
    screenshotTriggerSec: config.screenshot_trigger_sec,
    screenshotIntervalSec: config.screenshot_interval_sec,
    screenshotMode: config.screenshot_mode,
    screenshotHotkey: config.screenshot_hotkey,
  }),
  
  // 截图相关
  takeScreenshot: (appName: string) => invoke<ScreenshotResponse>('take_screenshot', { appName }),
  
  takeScreenshotArea: (appName: string, x: number, y: number, width: number, height: number) => 
    invoke<ScreenshotResponse>('take_screenshot_area', { appName, x, y, width, height }),
  
  getTodayScreenshots: () => invoke<string[]>('get_today_screenshots'),
  
  getScreenshotsByDate: (date: string) => invoke<string[]>('get_screenshots_by_date', { date }),
};
