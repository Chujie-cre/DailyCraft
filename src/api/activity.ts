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

export const activityApi = {
  getGroupedEvents: () => invoke<GroupedEvents>('get_today_events_grouped'),
  
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
  
  // 输入监听相关
  startInputListening: () => invoke('start_input_listening'),
  
  getInputStats: () => invoke<InputStats>('get_input_stats'),
  
  getIdleSeconds: () => invoke<number>('get_idle_seconds'),
  
  isInputListening: () => invoke<boolean>('is_input_listening'),
};
