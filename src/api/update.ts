import { invoke } from '@tauri-apps/api/core';

export interface ReleaseInfo {
  tag_name: string;
  name: string;
  body: string;
  html_url: string;
  published_at: string;
  download_url: string;
}

export interface UpdateCheckResult {
  has_update: boolean;
  current_version: string;
  latest_version: string | null;
  release_info: ReleaseInfo | null;
  skipped_reason: string | null;
}

export const updateApi = {
  /**
   * 检查更新
   * @param force 是否强制检查（忽略用户偏好）
   */
  async checkForUpdate(force: boolean = false): Promise<UpdateCheckResult> {
    return invoke('check_for_update', { force });
  },

  /**
   * 设置更新偏好
   * @param action 操作类型: 'ignore' | 'remind_tomorrow' | 'clear'
   * @param version 版本号（仅ignore操作需要）
   */
  async setUpdatePreference(action: string, version?: string): Promise<void> {
    return invoke('set_update_preference', { action, version });
  },

  /**
   * 获取当前版本
   */
  async getCurrentVersion(): Promise<string> {
    return invoke('get_current_version');
  },

  /**
   * 打开独立更新窗口
   */
  async openUpdateWindow(currentVersion: string, latestVersion: string, releaseInfo: ReleaseInfo): Promise<void> {
    return invoke('open_update_window', { 
      currentVersion, 
      latestVersion, 
      releaseInfo 
    });
  },

  /**
   * 下载更新安装包
   */
  async downloadUpdate(downloadUrl: string, version: string): Promise<string> {
    return invoke('download_update', { downloadUrl, version });
  },

  /**
   * 安装更新（启动安装程序并退出应用）
   */
  async installUpdate(filePath: string): Promise<void> {
    return invoke('install_update', { filePath });
  }
};
