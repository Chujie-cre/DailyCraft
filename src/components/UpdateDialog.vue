<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import { updateApi, type ReleaseInfo } from '@/api/update';
import { marked } from 'marked';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

// 检测是否是独立窗口模式
const isStandaloneWindow = ref(false);
const standaloneData = ref<{
  currentVersion: string;
  latestVersion: string;
  releaseInfo: ReleaseInfo | null;
}>({
  currentVersion: '',
  latestVersion: '',
  releaseInfo: null
});

onMounted(() => {
  // 检测URL参数判断是否是独立窗口
  const params = new URLSearchParams(window.location.search);
  if (params.has('current')) {
    isStandaloneWindow.value = true;
    standaloneData.value.currentVersion = params.get('current') || '';
    standaloneData.value.latestVersion = params.get('latest') || '';
    const releaseJson = params.get('release');
    if (releaseJson) {
      try {
        standaloneData.value.releaseInfo = JSON.parse(releaseJson);
      } catch (e) {
        console.error('解析release信息失败:', e);
      }
    }
  }
});

onUnmounted(() => {
  if (progressUnlisten) {
    progressUnlisten();
  }
});

const props = defineProps<{
  visible?: boolean;
  currentVersion?: string;
  latestVersion?: string;
  releaseInfo?: ReleaseInfo;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'updated'): void;
}>();

// 统一获取数据的计算属性
const currentVer = computed(() => isStandaloneWindow.value ? standaloneData.value.currentVersion : (props.currentVersion || ''));
const latestVer = computed(() => isStandaloneWindow.value ? standaloneData.value.latestVersion : (props.latestVersion || ''));
const release = computed(() => isStandaloneWindow.value ? standaloneData.value.releaseInfo : props.releaseInfo);
const isVisible = computed(() => isStandaloneWindow.value || props.visible);

const isProcessing = ref(false);
const isDownloading = ref(false);
const downloadProgress = ref(0);
const downloadedSize = ref(0);
const totalSize = ref(0);
const downloadedFilePath = ref('');
let progressUnlisten: UnlistenFn | null = null;

const formattedDate = computed(() => {
  if (!release.value?.published_at) return '';
  const date = new Date(release.value.published_at);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  });
});

const formattedBody = computed(() => {
  if (!release.value?.body) return '<p>暂无更新说明</p>';
  return marked(release.value.body) as string;
});

async function closeDialog() {
  if (isStandaloneWindow.value) {
    const win = getCurrentWindow();
    await win.close();
  } else {
    emit('close');
  }
}

async function handleUpdate() {
  if (!release.value?.download_url) {
    // 没有下载链接时跳转到发布页面
    if (release.value?.html_url) {
      await openUrl(release.value.html_url);
    }
    await closeDialog();
    return;
  }
  
  // 开始下载
  isDownloading.value = true;
  downloadProgress.value = 0;
  
  // 监听下载进度
  progressUnlisten = await listen<{ downloaded: number; total: number; percentage: number }>('download-progress', (event) => {
    downloadedSize.value = event.payload.downloaded;
    totalSize.value = event.payload.total;
    downloadProgress.value = event.payload.percentage;
  });
  
  try {
    const filePath = await updateApi.downloadUpdate(
      release.value.download_url,
      latestVer.value
    );
    downloadedFilePath.value = filePath;
    downloadProgress.value = 100;
  } catch (e) {
    console.error('下载失败:', e);
    isDownloading.value = false;
    // 下载失败时跳转到发布页面
    if (release.value?.html_url) {
      await openUrl(release.value.html_url);
    }
  }
}

async function handleInstall() {
  if (downloadedFilePath.value) {
    await updateApi.installUpdate(downloadedFilePath.value);
  }
}

function handleCancelDownload() {
  isDownloading.value = false;
  downloadProgress.value = 0;
  downloadedSize.value = 0;
  totalSize.value = 0;
  downloadedFilePath.value = '';
  if (progressUnlisten) {
    progressUnlisten();
    progressUnlisten = null;
  }
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

async function handleRemindTomorrow() {
  isProcessing.value = true;
  try {
    await updateApi.setUpdatePreference('remind_tomorrow');
    await closeDialog();
  } catch (e) {
    console.error('设置提醒失败:', e);
  } finally {
    isProcessing.value = false;
  }
}

async function handleIgnore() {
  isProcessing.value = true;
  try {
    await updateApi.setUpdatePreference('ignore', latestVer.value);
    await closeDialog();
  } catch (e) {
    console.error('忽略版本失败:', e);
  } finally {
    isProcessing.value = false;
  }
}
</script>

<template>
  <!-- 独立窗口模式：直接显示内容 -->
  <div v-if="isStandaloneWindow" class="standalone-update-page">
    <div class="update-dialog standalone">
      <div class="dialog-header">
        <div class="header-title-row">
          <img src="/icon.png" alt="DailyCraft" class="header-logo" />
          <h2>DailyCraft · 应用更新</h2>
        </div>
        <p class="version-subtitle">发现新版本</p>
        <p class="version-info">
          <span class="current">{{ currentVer }}</span>
          <span class="arrow">→</span>
          <span class="latest">{{ latestVer }}</span>
        </p>
      </div>
      <div class="dialog-body">
        <!-- 下载中：显示进度条 -->
        <div v-if="isDownloading" class="download-section-inline">
          <div class="download-title">正在下载更新...</div>
          <div class="download-progress-container">
            <div class="download-progress-bar" :style="{ width: downloadProgress + '%' }"></div>
          </div>
          <div class="download-info">
            <span>{{ downloadProgress.toFixed(0) }}%</span>
            <span>{{ formatSize(downloadedSize) }} / {{ formatSize(totalSize) }}</span>
          </div>
          <div class="download-buttons">
            <button v-if="downloadedFilePath" class="btn btn-primary btn-full" @click="handleInstall">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"></path>
                <path d="m9 12 2 2 4-4"></path>
              </svg>
              立即安装
            </button>
            <button v-else class="btn btn-ghost btn-full" @click="handleCancelDownload">
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="15" y1="9" x2="9" y2="15"></line>
                <line x1="9" y1="9" x2="15" y2="15"></line>
              </svg>
              取消下载
            </button>
          </div>
        </div>
        <!-- 未下载：显示更新日志 -->
        <template v-else>
          <div class="release-title">{{ release?.name || `v${latestVer}` }}</div>
          <div v-if="formattedDate" class="release-date">发布于 {{ formattedDate }}</div>
          <div class="release-notes">
            <div class="notes-label">更新内容：</div>
            <div class="notes-content markdown-body" v-html="formattedBody"></div>
          </div>
        </template>
      </div>
      
      <div v-if="!isDownloading" class="dialog-footer">
        <button class="btn btn-ghost" @click="handleIgnore" :disabled="isProcessing">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
          </svg>
          忽略此版本
        </button>
        <button class="btn btn-secondary" @click="handleRemindTomorrow" :disabled="isProcessing">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <polyline points="12 6 12 12 16 14"></polyline>
          </svg>
          明天提醒
        </button>
        <button class="btn btn-primary" @click="handleUpdate">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
          立即更新
        </button>
      </div>
    </div>
  </div>
  
  <!-- 弹窗模式：使用Teleport -->
  <Teleport v-else to="body">
    <div v-if="isVisible" class="update-dialog-overlay" @click.self="closeDialog">
      <div class="update-dialog">
        <div class="dialog-header">
          <div class="header-title-row">
            <img src="/icon.png" alt="DailyCraft" class="header-logo" />
            <h2>DailyCraft · 应用更新</h2>
          </div>
          <p class="version-subtitle">发现新版本</p>
          <p class="version-info">
            <span class="current">{{ currentVer }}</span>
            <span class="arrow">→</span>
            <span class="latest">{{ latestVer }}</span>
          </p>
        </div>

        <div class="dialog-body">
          <!-- 下载中：显示进度条 -->
          <div v-if="isDownloading" class="download-section-inline">
            <div class="download-title">正在下载更新...</div>
            <div class="download-progress-container">
              <div class="download-progress-bar" :style="{ width: downloadProgress + '%' }"></div>
            </div>
            <div class="download-info">
              <span>{{ downloadProgress.toFixed(0) }}%</span>
              <span>{{ formatSize(downloadedSize) }} / {{ formatSize(totalSize) }}</span>
            </div>
            <div class="download-buttons">
              <button v-if="downloadedFilePath" class="btn btn-primary btn-full" @click="handleInstall">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"></path>
                  <path d="m9 12 2 2 4-4"></path>
                </svg>
                立即安装
              </button>
              <button v-else class="btn btn-ghost btn-full" @click="handleCancelDownload">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="10"></circle>
                  <line x1="15" y1="9" x2="9" y2="15"></line>
                  <line x1="9" y1="9" x2="15" y2="15"></line>
                </svg>
                取消下载
              </button>
            </div>
          </div>
          <!-- 未下载：显示更新日志 -->
          <template v-else>
            <div class="release-title">{{ release?.name || `v${latestVer}` }}</div>
            <div v-if="formattedDate" class="release-date">发布于 {{ formattedDate }}</div>
            <div class="release-notes">
              <div class="notes-label">更新内容：</div>
              <div class="notes-content markdown-body" v-html="formattedBody"></div>
            </div>
          </template>
        </div>

        <div v-if="!isDownloading" class="dialog-footer">
          <button 
            class="btn btn-primary" 
            @click="handleUpdate"
            :disabled="isProcessing"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
              <polyline points="7 10 12 15 17 10"></polyline>
              <line x1="12" y1="15" x2="12" y2="3"></line>
            </svg>
            立即更新
          </button>
          <button 
            class="btn btn-secondary" 
            @click="handleRemindTomorrow"
            :disabled="isProcessing"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <polyline points="12 6 12 12 16 14"></polyline>
            </svg>
            明天提醒
          </button>
          <button 
            class="btn btn-ghost" 
            @click="handleIgnore"
            :disabled="isProcessing"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
            </svg>
            忽略此版本
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
/* 独立窗口模式样式 */
.standalone-update-page {
  width: 100vw;
  height: 100vh;
  background: #f8f9fa;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
}

.update-dialog.standalone {
  width: 100%;
  max-width: 100%;
  max-height: 100%;
  box-shadow: none;
  border: none;
}

.update-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  backdrop-filter: blur(4px);
}

@font-face {
  font-family: 'FZG_CN';
  src: url('/FZG_CN.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
}

.update-dialog {
  background: #ffffff;
  border-radius: 16px;
  width: 90%;
  max-width: 600px;
  max-height: 85vh;
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
  border: 1px solid #e5e7eb;
  animation: dialogSlideIn 0.3s ease;
  font-family: 'FZG_CN', sans-serif;
}

@keyframes dialogSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.dialog-header {
  padding: 20px 24px 16px;
  border-bottom: 1px solid #e5e7eb;
}

.header-title-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.header-logo {
  width: 28px;
  height: 28px;
  border-radius: 6px;
}

.dialog-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.version-subtitle {
  margin: 0 0 8px;
  font-size: 14px;
  color: #6b7280;
}

.version-info {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
  font-size: 14px;
}

.version-info .current {
  color: #6b7280;
  background: #f3f4f6;
  padding: 4px 8px;
  border-radius: 4px;
}

.version-info .arrow {
  color: #667eea;
}

.version-info .latest {
  color: #667eea;
  background: rgba(102, 126, 234, 0.2);
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: 600;
}

.dialog-body {
  height: 510px;
  padding: 20px 24px;
  overflow-y: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.dialog-body::-webkit-scrollbar {
  display: none;
}

.release-title {
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 4px;
}

.release-date {
  font-size: 12px;
  color: #888;
  margin-bottom: 16px;
}

.release-notes {
  background: #f9fafb;
  border-radius: 8px;
  padding: 12px;
  border: 1px solid #e5e7eb;
}

.notes-label {
  font-size: 12px;
  color: #888;
  margin-bottom: 8px;
}

.notes-content {
  font-size: 14px;
  color: #374151;
  line-height: 1.6;
}

.notes-content :deep(h1),
.notes-content :deep(h2),
.notes-content :deep(h3) {
  margin: 12px 0 8px;
  font-weight: 600;
  color: #1f2937;
}

.notes-content :deep(h1) { font-size: 18px; }
.notes-content :deep(h2) { font-size: 16px; }
.notes-content :deep(h3) { font-size: 14px; }

.notes-content :deep(p) {
  margin: 8px 0;
}

.notes-content :deep(ul),
.notes-content :deep(ol) {
  margin: 8px 0;
  padding-left: 20px;
}

.notes-content :deep(li) {
  margin: 4px 0;
}

.notes-content :deep(code) {
  background: #e5e7eb;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 13px;
}

.notes-content :deep(pre) {
  background: #f3f4f6;
  padding: 12px;
  border-radius: 6px;
  overflow-x: auto;
}

.notes-content :deep(a) {
  color: #667eea;
  text-decoration: none;
}

.notes-content :deep(a:hover) {
  text-decoration: underline;
}

.dialog-footer {
  padding: 16px 24px 24px;
  display: flex;
  flex-direction: row;
  gap: 10px;
  justify-content: flex-end;
}

.btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 10px 16px;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  white-space: nowrap;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-secondary {
  background: #f3f4f6;
  color: #374151;
  border: 1px solid #e5e7eb;
}

.btn-secondary:hover:not(:disabled) {
  background: #e5e7eb;
}

.btn-ghost {
  background: transparent;
  color: #6b7280;
}

.btn-ghost:hover:not(:disabled) {
  color: #374151;
  background: #f3f4f6;
}

.btn-full {
  width: 100%;
}

/* 下载进度样式 */
.download-section {
  padding: 16px 24px 24px;
}

.download-progress-container {
  width: 100%;
  height: 8px;
  background: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 12px;
}

.download-progress-bar {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.download-info {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: #6b7280;
  margin-bottom: 16px;
}

.download-section-inline {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  padding: 20px;
}

.download-title {
  font-size: 16px;
  font-weight: 500;
  color: #374151;
  margin-bottom: 24px;
}

.download-section-inline .download-progress-container {
  width: 100%;
  max-width: 400px;
}

.download-section-inline .download-info {
  width: 100%;
  max-width: 400px;
}

.download-buttons {
  width: 100%;
  max-width: 400px;
}
</style>
