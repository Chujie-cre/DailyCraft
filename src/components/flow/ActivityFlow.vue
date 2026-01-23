<script setup lang="ts">
import { computed, ref } from 'vue';
import { VueFlow } from '@vue-flow/core';
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

// 根据app名称找到对应的app_focus事件
function findAppForEvent(event: EventForDisplay, appEvents: EventForDisplay[]): EventForDisplay | null {
  if (!event.app || appEvents.length === 0) return null;
  
  // 根据app名称匹配，找到最近的同名应用事件
  const eventTime = parseTime(event.time_display);
  if (!eventTime) return null;
  
  let matchedApp: EventForDisplay | null = null;
  for (const app of appEvents) {
    if (app.app === event.app) {
      const appTime = parseTime(app.time_display);
      if (!appTime) continue;
      // 找到时间上早于或等于该事件的同名应用
      if (appTime.getTime() <= eventTime.getTime()) {
        matchedApp = app;
      }
    }
  }
  return matchedApp;
}

const nodes = computed(() => {
  if (!props.events) return [];
  
  const result: any[] = [];
  const xSpacing = 220;
  const ySpacing = 160;
  const labelX = -120;
  const appEvents = props.events.app_focus;
  const keyboardEvents = props.events.keyboard;
  const mouseEvents = props.events.mouse;
  const idleEvents = props.events.idle;
  
  // 第一行标题：应用焦点
  if (appEvents.length > 0) {
    result.push({
      id: 'label-app_focus',
      type: 'default',
      position: savedPositions.value['label-app_focus'] || { x: labelX, y: 20 },
      data: { label: `应用焦点 (${appEvents.length})` },
      style: {
        backgroundColor: '#3b82f6',
        color: 'white',
        fontWeight: 'bold',
        borderRadius: '8px',
        padding: '6px 12px',
        fontSize: '12px',
      },
    });
  }
  
  // 第一行：应用焦点事件（横向排列）
  appEvents.forEach((event, idx) => {
    let durationToNext = 0;
    if (idx < appEvents.length - 1) {
      durationToNext = calcDuration(event.time_display, appEvents[idx + 1].time_display);
    }
    
    const savedPos = savedPositions.value[event.id];
    result.push({
      id: event.id,
      type: 'activity',
      position: savedPos || { x: idx * xSpacing, y: 0 },
      data: {
        label: getEventLabel(event),
        eventType: event.event_type,
        time: event.time_display,
        app: event.app,
        windowTitle: event.window_title,
        exePath: event.exe_path,
        duration: formatDuration(durationToNext),
      },
    });
  });
  
  // 第二行标题：键盘输入
  if (keyboardEvents.length > 0) {
    result.push({
      id: 'label-keyboard',
      type: 'default',
      position: savedPositions.value['label-keyboard'] || { x: labelX, y: ySpacing + 20 },
      data: { label: `键盘输入 (${keyboardEvents.length})` },
      style: {
        backgroundColor: '#22c55e',
        color: 'white',
        fontWeight: 'bold',
        borderRadius: '8px',
        padding: '6px 12px',
        fontSize: '12px',
      },
    });
  }
  
  // 第二行：键盘事件
  if (appEvents.length > 0) {
    // 有应用事件时，关联到对应应用下方
    const keyboardByApp: Record<string, EventForDisplay[]> = {};
    keyboardEvents.forEach((event) => {
      const matchedApp = findAppForEvent(event, appEvents);
      const appId = matchedApp?.id || 'unknown';
      if (!keyboardByApp[appId]) keyboardByApp[appId] = [];
      keyboardByApp[appId].push(event);
    });
    
    appEvents.forEach((appEvent, appIdx) => {
      const kbEvents = keyboardByApp[appEvent.id] || [];
      kbEvents.forEach((event, subIdx) => {
        const savedPos = savedPositions.value[event.id];
        result.push({
          id: event.id,
          type: 'activity',
          position: savedPos || { x: appIdx * xSpacing + subIdx * 40, y: ySpacing },
          data: {
            label: getEventLabel(event),
            eventType: event.event_type,
            time: event.time_display,
            app: event.app,
            exePath: event.exe_path,
            keyCount: event.key_count,
          },
        });
      });
    });
  } else {
    // 没有应用事件时，独立显示键盘事件
    keyboardEvents.forEach((event, idx) => {
      const savedPos = savedPositions.value[event.id];
      result.push({
        id: event.id,
        type: 'activity',
        position: savedPos || { x: idx * xSpacing, y: ySpacing },
        data: {
          label: getEventLabel(event),
          eventType: event.event_type,
          time: event.time_display,
          app: event.app,
          exePath: event.exe_path,
          keyCount: event.key_count,
        },
      });
    });
  }
  
  // 第三行标题：鼠标操作
  if (mouseEvents.length > 0) {
    result.push({
      id: 'label-mouse',
      type: 'default',
      position: savedPositions.value['label-mouse'] || { x: labelX, y: ySpacing * 2 + 20 },
      data: { label: `鼠标操作 (${mouseEvents.length})` },
      style: {
        backgroundColor: '#f59e0b',
        color: 'white',
        fontWeight: 'bold',
        borderRadius: '8px',
        padding: '6px 12px',
        fontSize: '12px',
      },
    });
  }
  
  // 第三行：鼠标事件
  if (appEvents.length > 0) {
    // 有应用事件时，关联到对应应用下方
    const mouseByApp: Record<string, EventForDisplay[]> = {};
    mouseEvents.forEach((event) => {
      const matchedApp = findAppForEvent(event, appEvents);
      const appId = matchedApp?.id || 'unknown';
      if (!mouseByApp[appId]) mouseByApp[appId] = [];
      mouseByApp[appId].push(event);
    });
    
    appEvents.forEach((appEvent, appIdx) => {
      const msEvents = mouseByApp[appEvent.id] || [];
      msEvents.forEach((event, subIdx) => {
        const savedPos = savedPositions.value[event.id];
        result.push({
          id: event.id,
          type: 'activity',
          position: savedPos || { x: appIdx * xSpacing + subIdx * 40, y: ySpacing * 2 },
          data: {
            label: getEventLabel(event),
            eventType: event.event_type,
            time: event.time_display,
            app: event.app,
            exePath: event.exe_path,
            mouseDistance: event.mouse_distance,
            clickCount: event.click_count,
          },
        });
      });
    });
  } else {
    // 没有应用事件时，独立显示鼠标事件
    mouseEvents.forEach((event, idx) => {
      const savedPos = savedPositions.value[event.id];
      result.push({
        id: event.id,
        type: 'activity',
        position: savedPos || { x: idx * xSpacing, y: ySpacing * 2 },
        data: {
          label: getEventLabel(event),
          eventType: event.event_type,
          time: event.time_display,
          app: event.app,
          exePath: event.exe_path,
          mouseDistance: event.mouse_distance,
          clickCount: event.click_count,
        },
      });
    });
  }
  
  // 第四行标题：空闲状态
  if (idleEvents.length > 0) {
    result.push({
      id: 'label-idle',
      type: 'default',
      position: savedPositions.value['label-idle'] || { x: labelX, y: ySpacing * 3 + 20 },
      data: { label: `空闲状态 (${idleEvents.length})` },
      style: {
        backgroundColor: '#6b7280',
        color: 'white',
        fontWeight: 'bold',
        borderRadius: '8px',
        padding: '6px 12px',
        fontSize: '12px',
      },
    });
  }
  
  // 第四行：空闲事件
  idleEvents.forEach((event, idx) => {
    const savedPos = savedPositions.value[event.id];
    result.push({
      id: event.id,
      type: 'activity',
      position: savedPos || { x: idx * xSpacing, y: ySpacing * 3 },
      data: {
        label: getEventLabel(event),
        eventType: event.event_type,
        time: event.time_display,
      },
    });
  });
  
  return result;
});

const edges = computed(() => {
  if (!props.events) return [];
  
  const result: any[] = [];
  const appEvents = props.events.app_focus;
  const keyboardEvents = props.events.keyboard;
  const mouseEvents = props.events.mouse;
  const idleEvents = props.events.idle;
  
  // 标题到第一个事件的连接
  if (appEvents.length > 0) {
    result.push({
      id: 'edge-label-app-first',
      source: 'label-app_focus',
      target: appEvents[0].id,
      style: { stroke: '#3b82f6', strokeWidth: 1, opacity: 0.5 },
    });
  }
  if (keyboardEvents.length > 0) {
    result.push({
      id: 'edge-label-kb-first',
      source: 'label-keyboard',
      target: keyboardEvents[0].id,
      style: { stroke: '#22c55e', strokeWidth: 1, opacity: 0.5 },
    });
  }
  if (mouseEvents.length > 0) {
    result.push({
      id: 'edge-label-ms-first',
      source: 'label-mouse',
      target: mouseEvents[0].id,
      style: { stroke: '#f59e0b', strokeWidth: 1, opacity: 0.5 },
    });
  }
  if (idleEvents.length > 0) {
    result.push({
      id: 'edge-label-idle-first',
      source: 'label-idle',
      target: idleEvents[0].id,
      style: { stroke: '#6b7280', strokeWidth: 1, opacity: 0.5 },
    });
  }
  
  // 应用之间的横向连接（实线）
  for (let i = 0; i < appEvents.length - 1; i++) {
    result.push({
      id: `edge-app-${appEvents[i].id}-${appEvents[i + 1].id}`,
      source: appEvents[i].id,
      target: appEvents[i + 1].id,
      style: { stroke: '#3b82f6', strokeWidth: 2 },
    });
  }
  
  // 构建键盘和鼠标事件的映射
  const keyboardByApp: Record<string, EventForDisplay[]> = {};
  keyboardEvents.forEach((event) => {
    const matchedApp = findAppForEvent(event, appEvents);
    const appId = matchedApp?.id || 'unknown';
    if (!keyboardByApp[appId]) keyboardByApp[appId] = [];
    keyboardByApp[appId].push(event);
  });
  
  const mouseByApp: Record<string, EventForDisplay[]> = {};
  mouseEvents.forEach((event) => {
    const matchedApp = findAppForEvent(event, appEvents);
    const appId = matchedApp?.id || 'unknown';
    if (!mouseByApp[appId]) mouseByApp[appId] = [];
    mouseByApp[appId].push(event);
  });
  
  // 虚线连接：应用 → 键盘 → 鼠标
  appEvents.forEach((appEvent) => {
    const kbEvents = keyboardByApp[appEvent.id] || [];
    const msEvents = mouseByApp[appEvent.id] || [];
    
    // 应用 → 键盘
    kbEvents.forEach((kbEvent, idx) => {
      result.push({
        id: `edge-app-kb-${appEvent.id}-${kbEvent.id}`,
        source: appEvent.id,
        target: kbEvent.id,
        style: { stroke: '#22c55e', strokeDasharray: '5,5', opacity: 0.7 },
      });
      
      // 键盘 → 鼠标（如果有对应的鼠标事件）
      if (msEvents[idx]) {
        result.push({
          id: `edge-kb-ms-${kbEvent.id}-${msEvents[idx].id}`,
          source: kbEvent.id,
          target: msEvents[idx].id,
          style: { stroke: '#f59e0b', strokeDasharray: '5,5', opacity: 0.7 },
        });
      }
    });
    
    // 如果没有键盘事件但有鼠标事件，直接连接应用到鼠标
    if (kbEvents.length === 0 && msEvents.length > 0) {
      msEvents.forEach((msEvent) => {
        result.push({
          id: `edge-app-ms-${appEvent.id}-${msEvent.id}`,
          source: appEvent.id,
          target: msEvent.id,
          style: { stroke: '#f59e0b', strokeDasharray: '5,5', opacity: 0.7 },
        });
      });
    }
  });
  
  return result;
});

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
    
    <div v-if="!events || (events.app_focus.length === 0 && events.keyboard.length === 0 && events.mouse.length === 0 && events.idle.length === 0)" class="empty-state">
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
