<script setup lang="ts">
import Sidebar from "./components/layout/Sidebar.vue";
import Header from "./components/layout/Header.vue";
</script>

<template>
  <div class="app">
    <div class="sidebar-area">
      <Sidebar />
    </div>
    <div class="content-wrapper">
      <Header />
      <main class="main-content">
        <!-- 主内容区域 -->
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
}

.app {
  display: flex;
  min-height: 100vh;
  background-color: #f6f6f6;
}

.sidebar-area {
  width: 16.666%; /* 1/6 */
  position: relative;
}

.sidebar-area :deep(.radio-input) {
  position: absolute;
  left: 16px;
  top: 50%;
  transform: translateY(-50%);
}

.content-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  padding: 16px;
}
</style>