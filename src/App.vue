<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import Sidebar from "./components/layout/Sidebar.vue";
import Header from "./components/layout/Header.vue";
import Home from "./views/Home.vue";
import Dashboard from "./views/Dashboard.vue";
import Settings from "./views/Settings.vue";
import About from "./views/About.vue";
import Diary from "./views/Diary.vue";
import { activityApi } from './api/activity';

const currentPage = ref('home');

function handlePageChange(page: string) {
  currentPage.value = page;
}

// 全局追踪逻辑
let trackingInterval: number | null = null;
let idleCheckInterval: number | null = null;
let lastApp = '';
let lastTitle = '';
let lastExePath = '';
let lastIdleRecorded = 0; // 上次记录的空闲秒数
const IDLE_THRESHOLD = 300; // 5分钟空闲阈值（秒）

async function pollActiveWindow() {
  try {
    const info = await activityApi.getActiveWindow();
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
    }
  } catch (e) {
    console.error('追踪失败:', e);
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
    if (idleSeconds >= IDLE_THRESHOLD && idleSeconds !== lastIdleRecorded) {
      await activityApi.recordIdle(idleSeconds);
      lastIdleRecorded = idleSeconds;
    } else if (idleSeconds < IDLE_THRESHOLD) {
      lastIdleRecorded = 0; // 重置
    }
  } catch (e) {
    console.error('空闲检查失败:', e);
  }
}

function startGlobalTracking() {
  pollActiveWindow();
  trackingInterval = window.setInterval(pollActiveWindow, 1000);
  // 每30秒检查一次空闲状态
  idleCheckInterval = window.setInterval(checkIdleStatus, 30000);
}

onMounted(async () => {
  await activityApi.initTodayStorage();
  // 启动全局输入监听
  await activityApi.startInputListening();
  startGlobalTracking();
});

onUnmounted(() => {
  if (trackingInterval) {
    clearInterval(trackingInterval);
  }
  if (idleCheckInterval) {
    clearInterval(idleCheckInterval);
  }
});
</script>

<template>
  <div class="app">
    <div class="sidebar-area">
      <Sidebar @pageChange="handlePageChange" />
    </div>
    <Header class="fixed-header" />
    <div class="content-wrapper">
      <main class="main-content">
        <Home v-if="currentPage === 'home'" />
        <Dashboard v-else-if="currentPage === 'logs'" />
        <Diary v-else-if="currentPage === 'diary'" />
        <Settings v-else-if="currentPage === 'settings'" />
        <About v-else-if="currentPage === 'about'" />
      </main>
    </div>
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