<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { Handle, Position } from '@vue-flow/core';
import { activityApi } from '@/api/activity';

const props = defineProps<{
  data: {
    label: string;
    eventType: 'app_focus' | 'keyboard' | 'mouse' | 'idle';
    time: string;
    app?: string;
    windowTitle?: string;
    exePath?: string;
    keyCount?: number;
    mouseDistance?: number;
    clickCount?: number;
    duration?: string;
  };
}>();

const appIcon = ref<string | null>(null);

const nodeColors: Record<string, string> = {
  app_focus: '#3b82f6',
  keyboard: '#22c55e',
  mouse: '#f59e0b',
  idle: '#6b7280',
};

async function loadIcon() {
  if (props.data.exePath && props.data.eventType === 'app_focus') {
    try {
      appIcon.value = await activityApi.getAppIcon(props.data.exePath);
    } catch {
      appIcon.value = null;
    }
  }
}

onMounted(loadIcon);
watch(() => props.data.exePath, loadIcon);
</script>

<template>
  <div 
    class="activity-node"
    :style="{ borderColor: nodeColors[data.eventType] }"
  >
    <Handle type="target" :position="Position.Left" />
    
    <div class="node-header" :style="{ backgroundColor: nodeColors[data.eventType] }">
      <span class="icon">
        <img v-if="appIcon" :src="appIcon" alt="" class="app-icon-img" />
        <span v-else-if="data.eventType === 'app_focus'">{{ data.app?.charAt(0)?.toUpperCase() || 'A' }}</span>
        <span v-else-if="data.eventType === 'keyboard'">⌨️</span>
        <span v-else-if="data.eventType === 'mouse'">🖱️</span>
        <span v-else>💤</span>
      </span>
      <span class="time">{{ data.time }}</span>
    </div>
    
    <div class="node-body">
      <div class="label">{{ data.label }}</div>
      
      <div v-if="data.app" class="detail">
        <span class="detail-label">应用:</span>
        <span class="detail-value">{{ data.app }}</span>
      </div>
      
      <div v-if="data.windowTitle" class="detail">
        <span class="detail-label">窗口:</span>
        <span class="detail-value" :title="data.windowTitle">
          {{ data.windowTitle.length > 20 ? data.windowTitle.slice(0, 20) + '...' : data.windowTitle }}
        </span>
      </div>
      
      <div v-if="data.keyCount" class="detail">
        <span class="detail-label">按键:</span>
        <span class="detail-value">{{ data.keyCount }}次</span>
      </div>
      
      <div v-if="data.mouseDistance" class="detail">
        <span class="detail-label">移动:</span>
        <span class="detail-value">{{ Math.round(data.mouseDistance) }}px</span>
      </div>
      
      <div v-if="data.clickCount" class="detail">
        <span class="detail-label">点击:</span>
        <span class="detail-value">{{ data.clickCount }}次</span>
      </div>
    </div>
    
    <div v-if="data.duration" class="duration-badge">
      停留 {{ data.duration }}
    </div>
    
    <Handle type="source" :position="Position.Right" />
  </div>
</template>

<style scoped>
.activity-node {
  position: relative;
  background: white;
  border: 2px solid #ccc;
  border-radius: 8px;
  min-width: 160px;
  max-width: 200px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  font-size: 12px;
}

.node-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border-radius: 6px 6px 0 0;
  color: white;
  font-weight: 500;
}

.icon {
  font-size: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.app-icon-img {
  width: 18px;
  height: 18px;
  object-fit: contain;
}

.time {
  font-size: 11px;
  opacity: 0.9;
}

.node-body {
  padding: 8px 10px;
}

.label {
  font-weight: 600;
  color: #374151;
  margin-bottom: 6px;
}

.detail {
  display: flex;
  gap: 4px;
  margin-top: 4px;
  color: #6b7280;
  font-size: 11px;
}

.detail-label {
  color: #9ca3af;
}

.detail-value {
  color: #4b5563;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.duration-badge {
  position: absolute;
  bottom: -10px;
  right: 10px;
  background: #10b981;
  color: white;
  font-size: 10px;
  padding: 2px 8px;
  border-radius: 10px;
  font-weight: 500;
  white-space: nowrap;
}
</style>
