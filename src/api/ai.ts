import { invoke } from '@tauri-apps/api/core';

export interface AIConfig {
  api_key: string;
  model: string;
  base_url: string;
}

export interface DiaryState {
  is_generating: boolean;
  content: string;
  error: string;
  date: string;
}

export const aiApi = {
  async getConfig(): Promise<AIConfig> {
    return invoke('get_ai_config');
  },

  async saveConfig(config: AIConfig): Promise<void> {
    return invoke('save_ai_config', { config });
  },

  async generateDiary(activitiesJson: string, prompt: string): Promise<string> {
    return invoke('generate_diary', { activitiesJson, prompt });
  },

  async startDiaryGeneration(activitiesJson: string, prompt: string): Promise<void> {
    return invoke('start_diary_generation', { activitiesJson, prompt });
  },

  async getDiaryState(): Promise<DiaryState> {
    return invoke('get_diary_state');
  },

  async isDiaryGenerating(): Promise<boolean> {
    return invoke('is_diary_generating');
  },

  async saveDiary(date: string, content: string): Promise<string> {
    return invoke('save_diary', { date, content });
  },

  async getDiaryList(): Promise<string[]> {
    return invoke('get_diary_list');
  },

  async readDiary(date: string): Promise<string> {
    return invoke('read_diary', { date });
  }
};
