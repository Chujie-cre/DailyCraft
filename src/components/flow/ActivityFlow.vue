<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { VueFlow, useVueFlow } from '@vue-flow/core';
import { Background } from '@vue-flow/background';
import { Controls } from '@vue-flow/controls';
import '@vue-flow/core/dist/style.css';
import '@vue-flow/core/dist/theme-default.css';
import '@vue-flow/controls/dist/style.css';
import ActivityNode from './ActivityNode.vue';
import type { EventForDisplay, GroupedEvents } from '@/api/activity';

const props = defineProps<{
  events: GroupedEvents | null;
}>();

// 保存用户拖拽的节点位置
const savedPositions = ref<Record<string, { x: number; y: number }>>(
  JSON.parse(localStorage.getItem('flowNodePositions') || '{}')
);

function onNodeDragStop(event: any) {
  const node = event.node;
  if (node && node.id) {
    savedPositions.value[node.id] = { x: node.position.x, y: node.position.y };
    localStorage.setItem('flowNodePositions', JSON.stringify(savedPositions.value));
  }
}

const nodes = computed(() => {
  if (!props.events) return [];
  
  const result: any[] = [];
  let yOffset = 0;
  const xSpacing = 180;
  const ySpacing = 140;
  const itemsPerRow = 5;
  
  const categories = [
    { key: 'app_focus', label: '应用焦点', events: props.events.app_focus },
    { key: 'keyboard', label: '键盘输入', events: props.events.keyboard },
    { key: 'mouse', label: '鼠标操作', events: props.events.mouse },
    { key: 'idle', label: '空闲状态', events: props.events.idle },
  ];
  
  categories.forEach((category, catIdx) => {
    if (category.events.length === 0) return;
    
    // 类别标题节点
    const catId = `category-${category.key}`;
    const catSavedPos = savedPositions.value[catId];
    result.push({
      id: catId,
      type: 'default',
      position: catSavedPos || { x: 0, y: yOffset },
      data: { label: `${category.label} (${category.events.length})` },
      style: {
        backgroundColor: getCategoryColor(category.key),
        color: 'white',
        fontWeight: 'bold',
        borderRadius: '8px',
        padding: '8px 16px',
      },
    });
    
    // 事件节点 - 紧凑网格布局，每行5个
    category.events.forEach((event, idx) => {
      const row = Math.floor(idx / itemsPerRow);
      const col = idx % itemsPerRow;
      
      // 计算与下一个事件的间隔时长
      let durationToNext = 0;
      if (idx < category.events.length - 1) {
        durationToNext = calcDuration(event.time_display, category.events[idx + 1].time_display);
      }
      
      const savedPos = savedPositions.value[event.id];
      result.push({
        id: event.id,
        type: 'activity',
        position: savedPos || { x: (col + 1) * xSpacing, y: yOffset + row * ySpacing },
        data: {
          label: getEventLabel(event),
          eventType: event.event_type,
          time: event.time_display,
          app: event.app,
          windowTitle: event.window_title,
          exePath: event.exe_path,
          keyCount: event.key_count,
          mouseDistance: event.mouse_distance,
          clickCount: event.click_count,
          duration: formatDuration(durationToNext),
        },
      });
    });
    
    // 计算该类别占用的行数
    const rows = Math.ceil(category.events.length / itemsPerRow);
    yOffset += ySpacing * (rows + 0.5);
  });
  
  return result;
});

const edges = computed(() => {
  if (!props.events) return [];
  
  const result: any[] = [];
  const itemsPerRow = 5;
  
  const categories = [
    { key: 'app_focus', events: props.events.app_focus },
    { key: 'keyboard', events: props.events.keyboard },
    { key: 'mouse', events: props.events.mouse },
    { key: 'idle', events: props.events.idle },
  ];
  
  categories.forEach((category) => {
    const events = category.events;
    
    // 连接类别节点到第一个事件
    if (events.length > 0) {
      result.push({
        id: `edge-${category.key}-first`,
        source: `category-${category.key}`,
        target: events[0].id,
        animated: true,
        style: { stroke: getCategoryColor(category.key) },
      });
    }
    
    // 连接事件节点（按时间顺序）
    for (let i = 0; i < events.length - 1; i++) {
      result.push({
        id: `edge-${events[i].id}-${events[i + 1].id}`,
        source: events[i].id,
        target: events[i + 1].id,
        style: { stroke: getCategoryColor(category.key), opacity: 0.5 },
      });
    }
  });
  
  return result;
});

function getCategoryColor(key: string): string {
  const colors: Record<string, string> = {
    app_focus: '#3b82f6',
    keyboard: '#22c55e',
    mouse: '#f59e0b',
    idle: '#6b7280',
  };
  return colors[key] || '#888';
}

function getEventLabel(event: EventForDisplay): string {
  switch (event.event_type) {
    case 'app_focus':
      return event.app || '未知应用';
    case 'keyboard':
      return `键盘 ${event.key_count || 0}次`;
    case 'mouse':
      return `鼠标 ${event.click_count || 0}击`;
    case 'idle':
      return '空闲';
    default:
      return '未知';
  }
}

function parseTime(timeStr: string): Date | null {
  const match = timeStr.match(/(\d{2}):(\d{2}):(\d{2})/);
  if (!match) return null;
  const now = new Date();
  now.setHours(parseInt(match[1]), parseInt(match[2]), parseInt(match[3]), 0);
  return now;
}

function calcDuration(time1: string, time2: string): number {
  const t1 = parseTime(time1);
  const t2 = parseTime(time2);
  if (!t1 || !t2) return 0;
  return Math.floor((t2.getTime() - t1.getTime()) / 1000);
}

function formatDuration(seconds: number): string {
  if (seconds <= 0) return '';
  if (seconds < 60) return `${seconds}秒`;
  if (seconds < 3600) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return secs > 0 ? `${mins}分${secs}秒` : `${mins}分`;
  }
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  return mins > 0 ? `${hours}小时${mins}分` : `${hours}小时`;
}

</script>

<template>
  <div class="activity-flow-container">
    <VueFlow 
      :nodes="nodes" 
      :edges="edges"
      :default-viewport="{ zoom: 0.8 }"
      class="activity-flow"
      @node-drag-stop="onNodeDragStop"
    >
      <template #node-activity="nodeProps">
        <ActivityNode :data="nodeProps.data" />
      </template>
      <Background pattern-color="#aaa" :gap="20" />
      <Controls />
    </VueFlow>
    
    <div v-if="!events || (events.app_focus.length === 0 && events.keyboard.length === 0)" class="empty-state">
      <div class="empty-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 3v18h18"></path>
          <path d="M18 17V9"></path>
          <path d="M13 17V5"></path>
          <path d="M8 17v-3"></path>
        </svg>
      </div>
      <div class="empty-text">暂无活动数据</div>
      <div class="empty-hint">开始监控后，活动将显示在这里</div>
    </div>
  </div>
</template>

<style scoped>
.activity-flow-container {
  width: 100%;
  height: 100%;
  position: relative;
  background: #ffffff;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #e5e7eb;
}

.activity-flow {
  width: 100%;
  height: 100%;
}

.empty-state {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  color: #6b7280;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 18px;
  font-weight: 500;
  margin-bottom: 8px;
}

.empty-hint {
  font-size: 14px;
  color: #6b7280;
}
</style>
