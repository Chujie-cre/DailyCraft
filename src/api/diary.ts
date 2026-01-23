import { invoke } from '@tauri-apps/api/core';

export const diaryApi = {
  getDiaryList: () => invoke<string[]>('get_diary_list'),
  
  readDiary: (date: string) => invoke<string>('read_diary', { date }),
  
  saveDiary: (date: string, content: string) => invoke<string>('save_diary', { date, content }),
};
