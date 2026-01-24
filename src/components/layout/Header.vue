<template>
  <header class="header">
    <div class="header-left">
      <img src="/icon.png" alt="DailyCraft" class="logo-icon" />
      <h1 class="logo">DailyCraft</h1>
    </div>
    <div class="header-center">
      <span class="time-badge">
        <span class="time-text">{{ timeStr }}</span>
      </span>
    </div>
    <div class="header-right">
      <span class="date-display">{{ dateStr }}</span>
      <button class="header-btn" @click="handleSettingsClick">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const emit = defineEmits<{
  (e: 'goToSettings'): void;
}>();

const dateStr = ref('');
const timeStr = ref('');
let timer: number | null = null;

function updateDateTime() {
  const now = new Date();
  const dateOptions: Intl.DateTimeFormatOptions = { 
    year: 'numeric', 
    month: 'long', 
    day: 'numeric',
    weekday: 'long'
  };
  dateStr.value = now.toLocaleDateString('zh-CN', dateOptions);
  const h = String(now.getHours()).padStart(2, '0');
  const m = String(now.getMinutes()).padStart(2, '0');
  const s = String(now.getSeconds()).padStart(2, '0');
  timeStr.value = `${h}:${m}:${s}`;
}

onMounted(() => {
  updateDateTime();
  timer = window.setInterval(updateDateTime, 1000);
});

onUnmounted(() => {
  if (timer) {
    clearInterval(timer);
  }
});

function handleSettingsClick() {
  emit('goToSettings');
}
</script>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background-color: #fff;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  height: 60px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo-icon {
  width: 32px;
  height: 32px;
  object-fit: contain;
}

.logo {
  font-family: 'FZG', sans-serif;
  font-size: 22px;
  font-weight: 700;
  color: #333;
  margin: 0;
}

.header-center {
  display: flex;
  align-items: center;
}

.date-display {
  font-size: 14px;
  color: #666;
  margin-right: 8px;
}

.time-badge {
  display: inline-block;
  position: relative;
  padding: 5px 12px;
  border-radius: 14px;
  background: #fff;
}

.time-badge::before {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 14px;
  padding: 2px;
  background: linear-gradient(90deg, #ec4899, #f472b6, #ec4899);
  background-size: 200% 100%;
  -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  animation: border-shimmer 2s linear infinite;
}

@keyframes border-shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

.time-text {
  font-size: 13px;
  color: #374151;
  font-weight: 600;
  font-family: 'Consolas', monospace;
  position: relative;
  z-index: 1;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 8px;
  background-color: transparent;
  color: #666;
  cursor: pointer;
  transition: all 0.2s ease;
}

.header-btn:hover {
  background-color: #f0f0f0;
  color: #333;
}
</style>
