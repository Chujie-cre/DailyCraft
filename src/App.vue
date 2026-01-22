<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import Sidebar from "./components/layout/Sidebar.vue";
import Header from "./components/layout/Header.vue";
import Home from "./views/Home.vue";
import Dashboard from "./views/Dashboard.vue";
import Settings from "./views/Settings.vue";
import About from "./views/About.vue";
import { activityApi } from './api/activity';

const currentPage = ref('home');

function handlePageChange(page: string) {
  currentPage.value = page;
}

// 全局追踪逻辑
let trackingInterval: number | null = null;
let lastApp = '';
let lastTitle = '';

async function pollActiveWindow() {
  try {
    const info = await activityApi.getActiveWindow();
    if (info.app_name !== lastApp || info.window_title !== lastTitle) {
      await activityApi.recordAppFocus(info.app_name, info.window_title, info.exe_path);
      lastApp = info.app_name;
      lastTitle = info.window_title;
    }
  } catch (e) {
    console.error('追踪失败:', e);
  }
}

function startGlobalTracking() {
  pollActiveWindow();
  trackingInterval = window.setInterval(pollActiveWindow, 1000);
}

onMounted(async () => {
  await activityApi.initTodayStorage();
  startGlobalTracking();
});

onUnmounted(() => {
  if (trackingInterval) {
    clearInterval(trackingInterval);
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