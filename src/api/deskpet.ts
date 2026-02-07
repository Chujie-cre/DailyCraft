import { invoke } from '@tauri-apps/api/core';

export const deskpetApi = {
  /** 创建桌宠透明窗口 */
  async createPetWindow(): Promise<void> {
    await invoke('create_pet_window');
  },

  /** 关闭桌宠窗口 */
  async closePetWindow(): Promise<void> {
    await invoke('close_pet_window');
  },

  /** 获取全局鼠标位置 */
  async getMousePosition(): Promise<{ clientX: number; clientY: number } | null> {
    return await invoke('get_mouse_position');
  },

  /** 设置桌宠窗口鼠标穿透状态 */
  async setPetIgnoreCursor(ignore: boolean): Promise<void> {
    await invoke('set_pet_ignore_cursor', { ignore });
  },
};
