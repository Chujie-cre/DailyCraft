<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import Sidebar from "./components/layout/Sidebar.vue";
import Header from "./components/layout/Header.vue";
import Home from "./views/Home.vue";
import Dashboard from "./views/Dashboard.vue";
import Settings from "./views/Settings.vue";
import About from "./views/About.vue";
import Diary from "./views/Diary.vue";
import Screenshots from "./views/Screenshots.vue";
import Chat from "./views/Chat.vue";
import Notes from "./views/Notes.vue";
import UpdateDialog from './components/UpdateDialog.vue';

// 检测是否是独立更新窗口
const isUpdateWindow = computed(() => {
  return window.location.search.includes('current=');
});

import { activityApi, type AppConfig } from './api/activity';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { updateApi, type ReleaseInfo } from './api/update';

const currentPage = ref('home');

// 如果是更新窗口，不执行主窗口逻辑
if (isUpdateWindow.value) {
  // 更新窗口不需要主窗口的逻辑
}

// 更新检查相关
const showUpdateDialog = ref(false);
const updateInfo = ref<{
  currentVersion: string;
  latestVersion: string;
  releaseInfo: ReleaseInfo;
} | null>(null);

async function checkForUpdate() {
  if (isUpdateWindow.value) return; // 更新窗口不检查更新
  try {
    const result = await updateApi.checkForUpdate(false);
    if (result.has_update && result.release_info) {
      // 打开独立更新窗口
      await updateApi.openUpdateWindow(
        result.current_version,
        result.latest_version || '',
        result.release_info
      );
    }
  } catch (e) {
    console.error('检查更新失败:', e);
  }
}

function closeUpdateDialog() {
  showUpdateDialog.value = false;
}

function handlePageChange(page: string) {
  currentPage.value = page;
}

// 全局追踪逻辑
let trackingInterval: number | null = null;
let idleCheckInterval: number | null = null;
let screenshotInterval: number | null = null;
let configUnlisten: UnlistenFn | null = null;
let lastApp = '';
let lastTitle = '';
let lastExePath = '';
let lastIdleRecorded = 0;
let appStayStartTime = 0; // 应用停留开始时间
let lastScreenshotTime = 0; // 上次截图时间

// 配置（从后端加载）
let appConfig: AppConfig = {
  poll_interval_ms: 1000,
  idle_threshold_sec: 300,
  screenshot_enabled: false,
  screenshot_trigger_sec: 30,
  screenshot_interval_sec: 60,
  screenshot_mode: 'full_screen',
  screenshot_hotkey: 'Alt+]'
};

async function loadConfig() {
  try {
    appConfig = await activityApi.getAppConfig();
  } catch (e) {
    console.error('加载配置失败:', e);
  }
}

async function pollActiveWindow() {
  try {
    const info = await activityApi.getActiveWindow();
    const now = Date.now();
    
    // 只在应用切换时记录
    if (info.app_name !== lastApp || info.window_title !== lastTitle) {
      // 先记录上一个应用的输入统计（如果有）
      if (lastApp) {
        await recordInputStatsForApp();
      }
      
      // 记录新的应用焦点
      await activityApi.recordAppFocus(info.app_name, info.window_title, info.exe_path);
      lastApp = info.app_name;
      lastTitle = info.window_title;
      lastExePath = info.exe_path;
      
      // 重置应用停留计时
      appStayStartTime = now;
      lastScreenshotTime = 0;
    }
    
    // 自动截图逻辑
    if (appConfig.screenshot_enabled && lastApp) {
      const stayDuration = (now - appStayStartTime) / 1000;
      const sinceLastScreenshot = lastScreenshotTime ? (now - lastScreenshotTime) / 1000 : Infinity;
      
      // 首次截图：停留时间超过触发阈值
      if (stayDuration >= appConfig.screenshot_trigger_sec && lastScreenshotTime === 0) {
        await takeAutoScreenshot();
        lastScreenshotTime = now;
      }
      // 后续截图：间隔时间到达
      else if (lastScreenshotTime > 0 && sinceLastScreenshot >= appConfig.screenshot_interval_sec) {
        await takeAutoScreenshot();
        lastScreenshotTime = now;
      }
    }
  } catch (e) {
    console.error('追踪失败:', e);
  }
}

// 自动截图
async function takeAutoScreenshot() {
  try {
    const result = await activityApi.takeScreenshot(lastApp);
    if (!result.success) {
      // 静默失败
    }
  } catch (e) {
    console.error('截图异常:', e);
  }
}

// 在应用切换时记录输入统计
async function recordInputStatsForApp() {
  try {
    const stats = await activityApi.getInputStats();
    
    // 记录键盘事件（如果有按键）
    if (stats.key_count > 0) {
      await activityApi.recordKeyboard(stats.key_count, lastApp, lastTitle, lastExePath);
    }
    
    // 记录鼠标事件（如果有移动或点击）
    if (stats.mouse_distance > 0 || stats.click_count > 0) {
      await activityApi.recordMouse(stats.mouse_distance, stats.click_count, lastApp, lastTitle, lastExePath);
    }
  } catch (e) {
    console.error('输入统计失败:', e);
  }
}

// 独立检查空闲状态
async function checkIdleStatus() {
  try {
    const idleSeconds = await activityApi.getIdleSeconds();
    // 当空闲超过阈值且与上次记录不同时记录
    if (idleSeconds >= appConfig.idle_threshold_sec && idleSeconds !== lastIdleRecorded) {
      await activityApi.recordIdle(idleSeconds);
      lastIdleRecorded = idleSeconds;
    } else if (idleSeconds < appConfig.idle_threshold_sec) {
      lastIdleRecorded = 0; // 重置
    }
  } catch (e) {
    console.error('空闲检查失败:', e);
  }
}

function startGlobalTracking() {
  pollActiveWindow();
  // 使用配置的检查间隔
  trackingInterval = window.setInterval(pollActiveWindow, appConfig.poll_interval_ms);
  // 每30秒检查一次空闲状态
  idleCheckInterval = window.setInterval(checkIdleStatus, 30000);
}

// 重启追踪（配置变更后调用）
function restartTracking() {
  if (trackingInterval) {
    clearInterval(trackingInterval);
  }
  trackingInterval = window.setInterval(pollActiveWindow, appConfig.poll_interval_ms);
}

onMounted(async () => {
  // 先加载配置
  await loadConfig();
  await activityApi.initTodayStorage();
  // 启动全局输入监听
  await activityApi.startInputListening();
  startGlobalTracking();
  
  // 监听配置变更事件
  configUnlisten = await listen<AppConfig>('config-changed', (event) => {
    appConfig = event.payload;
    restartTracking();
  });
  
  // 启动时检查更新
  checkForUpdate();
});

onUnmounted(() => {
  if (trackingInterval) {
    clearInterval(trackingInterval);
  }
  if (idleCheckInterval) {
    clearInterval(idleCheckInterval);
  }
  if (screenshotInterval) {
    clearInterval(screenshotInterval);
  }
  if (configUnlisten) {
    configUnlisten();
  }
});
</script>

<template>
  <!-- 独立更新窗口 -->
  <UpdateDialog v-if="isUpdateWindow" />
  
  <!-- 主应用窗口 -->
  <div v-else class="app">
    <div class="sidebar-area">
      <Sidebar @pageChange="handlePageChange" />
    </div>
    <Header class="fixed-header" />
    <div class="content-wrapper">
      <main class="main-content">
        <Home v-if="currentPage === 'home'" />
        <Dashboard v-else-if="currentPage === 'logs'" />
        <Diary v-else-if="currentPage === 'diary'" />
        <Screenshots v-else-if="currentPage === 'screenshots'" />
        <Chat v-else-if="currentPage === 'chat'" />
        <Notes v-else-if="currentPage === 'notes'" />
        <Settings v-else-if="currentPage === 'settings'" />
        <About v-else-if="currentPage === 'about'" />
      </main>
    </div>
    
    <!-- 更新弹窗（保留，作为备用） -->
    <UpdateDialog
      v-if="updateInfo"
      :visible="showUpdateDialog"
      :current-version="updateInfo.currentVersion"
      :latest-version="updateInfo.latestVersion"
      :release-info="updateInfo.releaseInfo"
      @close="closeUpdateDialog"
    />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
}

.app {
  display: flex;
  min-height: 100vh;
  background-color: #f6f6f6;
  margin: 0;
  padding: 0;
}

.sidebar-area {
  width: 120px;
  min-width: 120px;
  position: fixed;
  left: 0;
  top: 0;
  height: 100vh;
  background: #fff;
  border-right: 1px solid #e5e7eb;
  z-index: 100;
}

.sidebar-area :deep(.radio-input) {
  height: 100%;
  background: transparent;
}

.content-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-left: 120px;
  margin-top: 60px;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  height: calc(100vh - 60px);
  min-height: 0;
}

.fixed-header {
  position: fixed;
  top: 0;
  left: 120px;
  right: 0;
  z-index: 99;
}
</style>