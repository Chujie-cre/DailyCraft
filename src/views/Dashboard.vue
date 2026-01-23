<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import ActivityFlow from '@/components/flow/ActivityFlow.vue';
import { activityApi, type GroupedEvents, type ActiveWindowInfo } from '@/api/activity';

const events = ref<GroupedEvents | null>(null);
const currentWindow = ref<ActiveWindowInfo | null>(null);
const eventCount = ref(0);

const filterType = ref<string>('all');
const filterDate = ref<string>(new Date().toISOString().split('T')[0]);
const currentPage = ref<number>(0);
const showDetailedRecords = ref<boolean>(false); // 是否显示详细应用记录（包含窗口切换）

const timeSlots = computed(() => {
  const slots: Array<{label: string; start: string; end: string}> = [];
  for (let i = 0; i < 24 * 6; i++) {
    const startHour = Math.floor(i / 6);
    const startMin = (i % 6) * 10;
    const endMin = startMin + 10;
    const endHour = endMin >= 60 ? startHour + 1 : startHour;
    const actualEndMin = endMin >= 60 ? 0 : endMin;
    
    const start = `${String(startHour).padStart(2, '0')}:${String(startMin).padStart(2, '0')}:00`;
    const end = `${String(endHour).padStart(2, '0')}:${String(actualEndMin).padStart(2, '0')}:00`;
    const label = `${String(startHour).padStart(2, '0')}:${String(startMin).padStart(2, '0')} - ${String(endHour).padStart(2, '0')}:${String(actualEndMin).padStart(2, '0')}`;
    
    slots.push({ label, start, end });
  }
  return slots;
});

const currentTimeSlot = computed(() => {
  const now = new Date();
  const hour = now.getHours();
  const min = Math.floor(now.getMinutes() / 10) * 10;
  return hour * 6 + min / 10;
});

const totalPages = computed(() => timeSlots.value.length);

const filteredEvents = computed<GroupedEvents | null>(() => {
  if (!events.value) return null;
  
  const result: GroupedEvents = {
    app_focus: [],
    keyboard: [],
    mouse: [],
    idle: []
  };
  
  const slot = timeSlots.value[currentPage.value];
  if (!slot) return result;
  
  const filterByTime = (items: typeof events.value.app_focus) => {
    return items.filter(item => {
      const time = item.time_display;
      return time >= slot.start && time < slot.end;
    });
  };
  
  if (filterType.value === 'all' || filterType.value === 'app_focus') {
    let appFocusEvents = filterByTime(events.value.app_focus);
    
    // 如果不显示详细记录，只保留应用切换（去除同一应用内的窗口切换）
    if (!showDetailedRecords.value) {
      const filtered: typeof appFocusEvents = [];
      let lastApp = '';
      for (const event of appFocusEvents) {
        if (event.app !== lastApp) {
          filtered.push(event);
          lastApp = event.app || '';
        }
      }
      appFocusEvents = filtered;
    }
    
    result.app_focus = appFocusEvents;
  }
  if (filterType.value === 'all' || filterType.value === 'keyboard') {
    result.keyboard = filterByTime(events.value.keyboard);
  }
  if (filterType.value === 'all' || filterType.value === 'mouse') {
    result.mouse = filterByTime(events.value.mouse);
  }
  if (filterType.value === 'all' || filterType.value === 'idle') {
    result.idle = filterByTime(events.value.idle);
  }
  
  return result;
});

function goToPage(page: number) {
  if (page >= 0 && page < totalPages.value) {
    currentPage.value = page;
  }
}

function goToCurrentTime() {
  currentPage.value = currentTimeSlot.value;
}

let refreshInterval: number | null = null;

async function loadEvents() {
  try {
    // 根据选择的日期加载事件
    const today = new Date().toISOString().split('T')[0];
    if (filterDate.value === today) {
      events.value = await activityApi.getGroupedEvents();
    } else {
      events.value = await activityApi.getGroupedEventsByDate(filterDate.value);
    }
    eventCount.value = await activityApi.getTodayEventCount();
    currentWindow.value = await activityApi.getActiveWindow();
  } catch (e) {
    console.error('加载事件失败:', e);
  }
}

// 监听日期变化，重新加载事件
watch(filterDate, async () => {
  await loadEvents();
  currentPage.value = 0; // 切换日期后重置到第一页
});

onMounted(async () => {
  await loadEvents();
  goToCurrentTime();
  // 每2秒刷新事件列表
  refreshInterval = window.setInterval(loadEvents, 2000);
});

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval);
  }
});
</script>

<template>
  <div class="dashboard">
    <div class="filter-bar">
      <div class="filter-group">
        <label>类型：</label>
        <select v-model="filterType">
          <option value="all">全部</option>
          <option value="app_focus">应用焦点</option>
          <option value="keyboard">键盘输入</option>
          <option value="mouse">鼠标操作</option>
          <option value="idle">空闲状态</option>
        </select>
      </div>
      <div class="filter-group">
        <label>日期：</label>
        <input type="date" v-model="filterDate" />
      </div>
      <div class="filter-group">
        <label>时间段：</label>
        <select v-model="currentPage" class="time-select">
          <option v-for="(slot, idx) in timeSlots" :key="idx" :value="idx">
            {{ slot.label }}
          </option>
        </select>
      </div>
      <button class="now-btn" @click="goToCurrentTime">当前时段</button>
      <div class="filter-group detail-toggle">
        <label class="toggle-label">
          <input type="checkbox" v-model="showDetailedRecords" />
          <span class="toggle-text">详细应用记录</span>
        </label>
      </div>
    </div>
    <ActivityFlow :events="filteredEvents" class="flow-area" />
    <div class="pagination">
      <button class="page-btn" :disabled="currentPage === 0" @click="goToPage(currentPage - 1)">
        ← 上一页
      </button>
      <span class="page-info">
        {{ timeSlots[currentPage]?.label || '' }}
        ({{ currentPage + 1 }} / {{ totalPages }})
      </span>
      <button class="page-btn" :disabled="currentPage >= totalPages - 1" @click="goToPage(currentPage + 1)">
        下一页 →
      </button>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.filter-bar {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 12px 16px;
  background: #fff;
  border-bottom: 1px solid #e5e7eb;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-group label {
  font-size: 14px;
  color: #374151;
  font-weight: 500;
}

.filter-group select,
.filter-group input {
  padding: 6px 10px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  background: #fff;
}

.filter-group select:focus,
.filter-group input:focus {
  outline: none;
  border-color: #3b82f6;
}

.clear-filter {
  padding: 6px 12px;
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.clear-filter:hover {
  background: #e5e7eb;
}

.flow-area {
  flex: 1;
  min-height: 0;
}

.time-select {
  min-width: 160px;
}

.now-btn {
  padding: 6px 12px;
  background: #3b82f6;
  border: none;
  border-radius: 6px;
  color: #fff;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.now-btn:hover {
  background: #2563eb;
}

.detail-toggle {
  margin-left: auto;
}

.toggle-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.toggle-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: #3b82f6;
}

.toggle-text {
  font-size: 14px;
  color: #374151;
}

.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 12px 16px;
  background: #fff;
  border-top: 1px solid #e5e7eb;
}

.page-btn {
  padding: 8px 16px;
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.page-btn:hover:not(:disabled) {
  background: #e5e7eb;
}

.page-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.page-info {
  font-size: 14px;
  color: #374151;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: #1a1a2e;
  border-bottom: 1px solid #2d2d44;
}

.header-info {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-info h2 {
  margin: 0;
  font-size: 18px;
  color: #fff;
}

.event-count {
  font-size: 13px;
  color: #9ca3af;
  background: #2d2d44;
  padding: 4px 10px;
  border-radius: 12px;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.current-window {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: #9ca3af;
  background: #2d2d44;
  padding: 6px 12px;
  border-radius: 6px;
}

.current-window .label {
  color: #6b7280;
}

.current-window .app {
  color: #3b82f6;
  font-weight: 500;
}

.tracking-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  background: #3b82f6;
  color: white;
}

.tracking-btn:hover {
  background: #2563eb;
}

.tracking-btn.active {
  background: #ef4444;
}

.tracking-btn.active:hover {
  background: #dc2626;
}

.tracking-btn .icon {
  font-size: 12px;
}

.refresh-btn {
  padding: 8px 12px;
  border: 1px solid #3d3d5c;
  border-radius: 6px;
  background: transparent;
  color: #9ca3af;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.refresh-btn:hover {
  background: #2d2d44;
  color: #fff;
}

.dashboard-content {
  flex: 1;
  padding: 16px;
  overflow: hidden;
  min-height: 500px;
}
</style>
