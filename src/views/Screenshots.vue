<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { activityApi } from '@/api/activity';
import { convertFileSrc } from '@tauri-apps/api/core';

interface ScreenshotInfo {
  path: string;
  time: string;
  appName: string;
  timestamp: number;
  iconBase64?: string;
}

const screenshots = ref<ScreenshotInfo[]>([]);
const currentIndex = ref(0);
function getTodayDate() {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}
const selectedDate = ref(getTodayDate());
const isLoading = ref(false);
const hoveredIndex = ref(-1);
const timelineBarsRef = ref<HTMLElement | null>(null);
const timelineBarsInnerRef = ref<HTMLElement | null>(null);
const isDragging = ref(false);

const currentScreenshot = computed(() => {
  if (screenshots.value.length === 0) return null;
  return screenshots.value[currentIndex.value];
});

const currentImageSrc = computed(() => {
  if (!currentScreenshot.value) return '';
  return convertFileSrc(currentScreenshot.value.path);
});

// 计算时间轴范围（基于实际截图时间）
const timelineRange = computed(() => {
  if (screenshots.value.length === 0) {
    return { start: 0, end: 86400, labels: ['00:00', '06:00', '12:00', '18:00', '24:00'] };
  }
  
  const timestamps = screenshots.value.map(s => s.timestamp);
  const minTime = Math.min(...timestamps);
  const maxTime = Math.max(...timestamps);
  
  // 扩展范围，前后各加5分钟
  const padding = 300;
  const start = Math.max(0, minTime - padding);
  const end = Math.min(86400, maxTime + padding);
  
  // 生成时间标签
  const range = end - start;
  const labels: string[] = [];
  const step = range / 4;
  for (let i = 0; i <= 4; i++) {
    const t = start + step * i;
    const h = Math.floor(t / 3600);
    const m = Math.floor((t % 3600) / 60);
    labels.push(`${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}`);
  }
  
  return { start, end, labels };
});

function parseScreenshotInfo(filepath: string): ScreenshotInfo {
  const filename = filepath.split(/[/\\]/).pop() || '';
  // 文件名格式: HH-MM-SS_appname.jpg
  const match = filename.match(/^(\d{2})-(\d{2})-(\d{2})_(.+)\.(jpg|png)$/);
  
  if (match) {
    const [, hour, minute, second, appName] = match;
    const time = `${hour}:${minute}:${second}`;
    const timestamp = parseInt(hour) * 3600 + parseInt(minute) * 60 + parseInt(second);
    return {
      path: filepath,
      time,
      appName: appName.replace(/_/g, ' '),
      timestamp,
      iconBase64: undefined
    };
  }
  
  return {
    path: filepath,
    time: '未知',
    appName: '未知',
    timestamp: 0,
    iconBase64: undefined
  };
}

async function loadScreenshots() {
  isLoading.value = true;
  try {
    const paths = await activityApi.getScreenshotsByDate(selectedDate.value);
    const parsed = paths
      .map(parseScreenshotInfo)
      .sort((a, b) => a.timestamp - b.timestamp);
    
    // 异步加载应用图标
    for (const shot of parsed) {
      if (shot.appName && shot.appName !== '未知') {
        try {
          const icon = await activityApi.getIconForApp(shot.appName);
          if (icon) {
            shot.iconBase64 = icon;
          }
        } catch {
          // 忽略图标加载失败
        }
      }
    }
    
    screenshots.value = parsed;
    
    if (screenshots.value.length > 0) {
      currentIndex.value = 0;
    }
  } catch (e) {
    console.error('加载截图失败:', e);
  } finally {
    isLoading.value = false;
  }
}

function prevScreenshot() {
  if (currentIndex.value > 0) {
    currentIndex.value--;
  }
}

function nextScreenshot() {
  if (currentIndex.value < screenshots.value.length - 1) {
    currentIndex.value++;
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'ArrowLeft') {
    prevScreenshot();
  } else if (event.key === 'ArrowRight') {
    nextScreenshot();
  }
}

function getIndexFromMousePosition(event: MouseEvent): number {
  if (!timelineBarsInnerRef.value || screenshots.value.length === 0) return 0;
  const bars = timelineBarsInnerRef.value.children;
  if (bars.length === 0) return 0;
  
  // 直接计算鼠标位置对应的bar索引
  for (let i = 0; i < bars.length; i++) {
    const barRect = bars[i].getBoundingClientRect();
    if (event.clientX <= barRect.right) {
      return i;
    }
  }
  return bars.length - 1;
}

function handleTimelineMouseDown(event: MouseEvent) {
  isDragging.value = true;
  currentIndex.value = getIndexFromMousePosition(event);
  
  const handleMouseMove = (e: MouseEvent) => {
    if (isDragging.value) {
      currentIndex.value = getIndexFromMousePosition(e);
    }
  };
  
  const handleMouseUp = () => {
    isDragging.value = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  };
  
  window.addEventListener('mousemove', handleMouseMove);
  window.addEventListener('mouseup', handleMouseUp);
}

function handleTimelineMouseMove(event: MouseEvent) {
  if (isDragging.value) {
    currentIndex.value = getIndexFromMousePosition(event);
  }
}

function getTooltipStyle(): { left: string } {
  if (!timelineBarsInnerRef.value || hoveredIndex.value < 0) {
    return { left: '0px' };
  }
  const bars = timelineBarsInnerRef.value.children;
  if (bars.length === 0 || hoveredIndex.value >= bars.length) {
    return { left: '0px' };
  }
  const bar = bars[hoveredIndex.value] as HTMLElement;
  const wrapperRect = timelineBarsRef.value?.getBoundingClientRect();
  const barRect = bar.getBoundingClientRect();
  if (!wrapperRect) return { left: '0px' };
  const left = barRect.left - wrapperRect.left + barRect.width / 2;
  return { left: `${left}px` };
}

watch(selectedDate, () => {
  loadScreenshots();
});

onMounted(() => {
  // 每次组件挂载时更新日期到今天
  selectedDate.value = getTodayDate();
  loadScreenshots();
  window.addEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div class="screenshots-container">
    <div class="screenshots-header">
      <h2>
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="header-icon">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
        截图时间轴
      </h2>
      <div class="date-picker">
        <input type="date" v-model="selectedDate" class="date-input" />
      </div>
    </div>

    <div v-if="isLoading" class="loading-state">
      <div class="cat-loader">
        <div class="cat-wrapper">
          <div class="cat-container">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 733 673" class="cat-body">
              <path fill="#212121" d="M111.002 139.5C270.502 -24.5001 471.503 2.4997 621.002 139.5C770.501 276.5 768.504 627.5 621.002 649.5C473.5 671.5 246 687.5 111.002 649.5C-23.9964 611.5 -48.4982 303.5 111.002 139.5Z"></path>
              <path fill="#212121" d="M184 9L270.603 159H97.3975L184 9Z"></path>
              <path fill="#212121" d="M541 0L627.603 150H454.397L541 0Z"></path>
            </svg>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 158 564" class="cat-tail">
              <path fill="#191919" d="M5.97602 76.066C-11.1099 41.6747 12.9018 0 51.3036 0V0C71.5336 0 89.8636 12.2558 97.2565 31.0866C173.697 225.792 180.478 345.852 97.0691 536.666C89.7636 553.378 73.0672 564 54.8273 564V564C16.9427 564 -5.4224 521.149 13.0712 488.085C90.2225 350.15 87.9612 241.089 5.97602 76.066Z"></path>
            </svg>
            <div class="cat-zzz">
              <span class="zzz-big">Z</span>
              <span class="zzz-small">Z</span>
            </div>
          </div>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 500 126" class="cat-wall">
            <line stroke-width="6" stroke="#7C7C7C" y2="3" x2="450" y1="3" x1="50"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="85" x2="400" y1="85" x1="100"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="122" x2="375" y1="122" x1="125"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="43" x2="500" y1="43"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="1.99391" x2="115.5" y1="43.0061" x1="115.5"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="2.00002" x2="189" y1="43.0122" x1="189"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="2.00612" x2="262.5" y1="43.0183" x1="262.5"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="2.01222" x2="336" y1="43.0244" x1="336"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="2.01833" x2="409.5" y1="43.0305" x1="409.5"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="43" x2="153" y1="84.0122" x1="153"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="43" x2="228" y1="84.0122" x1="228"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="43" x2="303" y1="84.0122" x1="303"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="43" x2="378" y1="84.0122" x1="378"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="84" x2="192" y1="125.012" x1="192"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="84" x2="267" y1="125.012" x1="267"></line>
            <line stroke-width="6" stroke="#7C7C7C" y2="84" x2="342" y1="125.012" x1="342"></line>
          </svg>
        </div>
      </div>
    </div>

    <div v-else-if="screenshots.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
      </div>
      <p>当天没有截图记录</p>
      <p class="empty-hint">开启自动截图后，这里会显示当天的屏幕截图</p>
    </div>

    <template v-else>
      <div class="screenshot-viewer">
        <div class="screenshot-info">
          <span class="time-badge">{{ currentScreenshot?.time }}</span>
          <span class="app-badge" :title="currentScreenshot?.appName">
            <img 
              v-if="currentScreenshot?.iconBase64" 
              :src="currentScreenshot.iconBase64" 
              class="app-icon"
              alt=""
            />
            <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="app-icon-fallback">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
              <line x1="8" y1="21" x2="16" y2="21"></line>
              <line x1="12" y1="17" x2="12" y2="21"></line>
            </svg>
            {{ currentScreenshot?.appName }}
          </span>
          <span class="count-badge">{{ currentIndex + 1 }} / {{ screenshots.length }}</span>
        </div>
        
        <div class="screenshot-display">
          <button class="nav-btn prev" @click="prevScreenshot" :disabled="currentIndex === 0">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="15 18 9 12 15 6"></polyline>
            </svg>
          </button>
          
          <div class="image-container">
            <img :src="currentImageSrc" :alt="currentScreenshot?.appName" />
          </div>
          
          <button class="nav-btn next" @click="nextScreenshot" :disabled="currentIndex === screenshots.length - 1">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </button>
        </div>
      </div>

      <div class="timeline-container">
        <div 
          class="timeline-bars-wrapper"
          ref="timelineBarsRef"
          @mousedown="handleTimelineMouseDown"
          @mousemove="handleTimelineMouseMove"
          @mouseleave="hoveredIndex = -1"
        >
          <div class="timeline-bars" ref="timelineBarsInnerRef">
            <div 
              v-for="(shot, index) in screenshots" 
              :key="shot.path"
              class="timeline-bar"
              :class="{ active: index === currentIndex, past: index < currentIndex, hovered: index === hoveredIndex }"
              @mouseenter="hoveredIndex = index"
              @click.stop="currentIndex = index"
            ></div>
          </div>
          <div 
            v-if="hoveredIndex >= 0 && screenshots[hoveredIndex]" 
            class="timeline-tooltip"
            :style="getTooltipStyle()"
          >
            {{ screenshots[hoveredIndex].time }}
          </div>
        </div>
        <div class="timeline-info">
          <span class="timeline-time">{{ timelineRange.labels[0] }}</span>
          <span class="timeline-current">{{ currentScreenshot?.time }}</span>
          <span class="timeline-time">{{ timelineRange.labels[timelineRange.labels.length - 1] }}</span>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.screenshots-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 20px;
  background: #f8fafc;
  overflow: hidden;
  box-sizing: border-box;
}

.screenshots-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.screenshots-header h2 {
  margin: 0;
  font-size: 20px;
  color: #1f2937;
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-icon {
  color: #3b82f6;
}

.date-input {
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  color: #374151;
  background: white;
}

.loading-state,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #6b7280;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #e5e7eb;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 12px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.empty-icon {
  margin-bottom: 16px;
  color: #d1d5db;
}

.empty-hint {
  font-size: 13px;
  color: #9ca3af;
  margin-top: 8px;
}

.screenshot-viewer {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.screenshot-info {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.time-badge,
.app-badge,
.count-badge {
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 13px;
  font-weight: 500;
}

.time-badge {
  background: #dbeafe;
  color: #1d4ed8;
}

.app-badge {
  background: #dcfce7;
  color: #16a34a;
  display: flex;
  align-items: center;
  gap: 6px;
}

.app-icon {
  width: 18px;
  height: 18px;
  object-fit: contain;
}

.app-icon-fallback {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.count-badge {
  background: #f3f4f6;
  color: #6b7280;
}

.screenshot-display {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  min-height: 0;
}

.nav-btn {
  width: 48px;
  height: 48px;
  border: none;
  border-radius: 50%;
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #374151;
  transition: all 0.2s;
  flex-shrink: 0;
}

.nav-btn:hover:not(:disabled) {
  background: #f3f4f6;
  transform: scale(1.05);
}

.nav-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.image-container {
  flex: 1;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1f2937;
  border-radius: 12px;
  overflow: hidden;
}

.image-container img {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.timeline-container {
  margin-top: 20px;
  padding: 16px 20px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  max-width: 100%;
}

.timeline-bars-wrapper {
  position: relative;
  width: 100%;
  overflow: hidden;
  cursor: pointer;
  user-select: none;
}

.timeline-bars {
  display: flex;
  align-items: flex-end;
  gap: 0;
  height: 60px;
  padding: 0;
  width: 100%;
  overflow: hidden;
}

.timeline-tooltip {
  position: absolute;
  top: -30px;
  transform: translateX(-50%);
  background: #1f2937;
  color: white;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
  pointer-events: none;
  z-index: 20;
}

.timeline-tooltip::after {
  content: '';
  position: absolute;
  bottom: -4px;
  left: 50%;
  transform: translateX(-50%);
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-top: 5px solid #1f2937;
}

.timeline-bar {
  flex: 1 1 0;
  min-width: 1px;
  max-width: 8px;
  height: 100%;
  background: #e5e7eb;
  border-radius: 2px 2px 0 0;
  cursor: pointer;
  transition: all 0.15s ease;
}

.timeline-bar:hover,
.timeline-bar.hovered {
  background: #9ca3af;
  transform: scaleY(1.1);
  transform-origin: bottom;
}

.timeline-bar.past {
  background: #1f2937;
}

.timeline-bar.active {
  background: #3b82f6;
  box-shadow: 0 0 8px rgba(59, 130, 246, 0.5);
}

.timeline-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
  font-size: 12px;
}

.timeline-time {
  color: #9ca3af;
}

.timeline-current {
  font-size: 14px;
  font-weight: 600;
  color: #3b82f6;
  background: #eff6ff;
  padding: 4px 12px;
  border-radius: 12px;
}

/* 猫睡觉加载动画 */
.cat-loader {
  display: flex;
  align-items: center;
  justify-content: center;
}

.cat-wrapper {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.cat-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.cat-body {
  width: 80px;
}

.cat-tail {
  position: absolute;
  width: 17px;
  top: 50%;
  animation: tail-wag 0.5s ease-in infinite alternate-reverse;
  transform-origin: top;
}

@keyframes tail-wag {
  0% { transform: rotateZ(60deg); }
  50% { transform: rotateZ(0deg); }
  100% { transform: rotateZ(-20deg); }
}

.cat-wall {
  width: 300px;
}

.cat-zzz {
  display: flex;
  flex-direction: column;
  width: 50px;
  position: absolute;
  margin: 0px 0px 100px 120px;
}

.zzz-small {
  color: #1f2937;
  font-weight: 700;
  font-size: 15px;
  animation: zzz-fade 2s linear infinite;
}

.zzz-big {
  color: #1f2937;
  font-weight: 700;
  font-size: 25px;
  margin-left: 10px;
  animation: zzz-fade 2.3s linear infinite;
}

@keyframes zzz-fade {
  0% { color: transparent; }
  50% { color: #1f2937; }
  100% { color: transparent; }
}
</style>
