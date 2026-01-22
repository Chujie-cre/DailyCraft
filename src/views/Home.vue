<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { activityApi } from '@/api/activity';
import VChart from 'vue-echarts';
import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { PieChart, BarChart, LineChart } from 'echarts/charts';
import { TitleComponent, TooltipComponent, LegendComponent, GridComponent } from 'echarts/components';

use([CanvasRenderer, PieChart, BarChart, LineChart, TitleComponent, TooltipComponent, LegendComponent, GridComponent]);

// 图表引用
const pieChartRef = ref<InstanceType<typeof VChart> | null>(null);
const barChartRef = ref<InstanceType<typeof VChart> | null>(null);

// 窗口resize处理
let resizeTimer: number | null = null;
function handleResize() {
  if (resizeTimer) clearTimeout(resizeTimer);
  resizeTimer = window.setTimeout(() => {
    pieChartRef.value?.chart?.resize();
    barChartRef.value?.chart?.resize();
  }, 100);
}

interface AppRecord {
  app: string;
  count: number;
  totalDuration: number; // 总使用时长（秒）
  color: string;
  icon: string | null;
  exePath: string;
}

const eventCount = ref(0);
const recentApps = ref<AppRecord[]>([]);
const appIconCache = ref<Map<string, string | null>>(new Map());

const appColors = ['#e8d5f2', '#fcd9c4', '#c5e8f7', '#d4f5d4', '#ffeaa7', '#fab1a0'];

async function loadAppIcon(exePath: string): Promise<string | null> {
  if (!exePath) return null;
  if (appIconCache.value.has(exePath)) {
    return appIconCache.value.get(exePath) || null;
  }
  try {
    const icon = await activityApi.getAppIcon(exePath);
    appIconCache.value.set(exePath, icon);
    return icon;
  } catch {
    appIconCache.value.set(exePath, null);
    return null;
  }
}

function parseTimeDisplay(timeStr: string): Date | null {
  // 解析时间字符串如 "14:30:25"
  const match = timeStr.match(/(\d{2}):(\d{2}):(\d{2})/);
  if (!match) return null;
  const now = new Date();
  now.setHours(parseInt(match[1]), parseInt(match[2]), parseInt(match[3]), 0);
  return now;
}

function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}秒`;
  if (seconds < 3600) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return secs > 0 ? `${mins}分${secs}秒` : `${mins}分钟`;
  }
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  return mins > 0 ? `${hours}小时${mins}分` : `${hours}小时`;
}

async function loadStats() {
  try {
    eventCount.value = await activityApi.getTodayEventCount();
    const events = await activityApi.getGroupedEvents();
    
    // 统计最近应用，计算使用时长
    const appMap = new Map<string, { count: number; exePath: string; totalDuration: number }>();
    const focusEvents = events.app_focus;
    
    // 按时间排序事件
    const sortedEvents = [...focusEvents].sort((a, b) => {
      const timeA = parseTimeDisplay(a.time_display);
      const timeB = parseTimeDisplay(b.time_display);
      if (!timeA || !timeB) return 0;
      return timeA.getTime() - timeB.getTime();
    });
    
    // 计算每个应用的使用时长
    for (let i = 0; i < sortedEvents.length; i++) {
      const e = sortedEvents[i];
      if (!e.app) continue;
      
      // 计算停留时长：到下一个事件的时间差
      let duration = 0;
      if (i < sortedEvents.length - 1) {
        const currentTime = parseTimeDisplay(e.time_display);
        const nextTime = parseTimeDisplay(sortedEvents[i + 1].time_display);
        if (currentTime && nextTime) {
          duration = Math.floor((nextTime.getTime() - currentTime.getTime()) / 1000);
          // 限制单次最大时长为30分钟（避免异常数据）
          if (duration > 1800) duration = 0;
        }
      }
      
      const existing = appMap.get(e.app);
      if (existing) {
        existing.count++;
        existing.totalDuration += duration;
        if (!existing.exePath && e.exe_path) {
          existing.exePath = e.exe_path;
        }
      } else {
        appMap.set(e.app, { count: 1, exePath: e.exe_path || '', totalDuration: duration });
      }
    }
    
    const apps = Array.from(appMap.entries())
      .sort((a, b) => b[1].totalDuration - a[1].totalDuration) // 按使用时长排序
      .slice(0, 8);
    
    // 加载图标
    const appsWithIcons: AppRecord[] = [];
    for (let i = 0; i < apps.length; i++) {
      const [app, data] = apps[i];
      const icon = data.exePath ? await loadAppIcon(data.exePath) : null;
      appsWithIcons.push({
        app,
        count: data.count,
        totalDuration: data.totalDuration,
        color: appColors[i % appColors.length],
        icon,
        exePath: data.exePath
      });
    }
    
    recentApps.value = appsWithIcons;
  } catch (e) {
    console.error('加载统计失败:', e);
  }
}

// 饼图配置 - 应用使用时长分布
const pieChartOption = computed(() => ({
  title: {
    text: '应用使用时长分布',
    left: 'center',
    textStyle: { fontSize: 14, color: '#374151' }
  },
  tooltip: {
    trigger: 'item',
    formatter: (params: any) => `${params.name}: ${formatDuration(params.value)}`
  },
  series: [{
    type: 'pie',
    radius: ['40%', '70%'],
    avoidLabelOverlap: false,
    itemStyle: {
      borderRadius: 6,
      borderColor: '#fff',
      borderWidth: 2
    },
    label: {
      show: true,
      formatter: '{b}'
    },
    data: recentApps.value.map((app, idx) => ({
      value: app.totalDuration,
      name: app.app,
      itemStyle: { color: ['#3b82f6', '#22c55e', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#ec4899', '#84cc16'][idx % 8] }
    }))
  }]
}));

// 柱状图配置 - 应用切换次数
const barChartOption = computed(() => ({
  title: {
    text: '应用切换次数',
    left: 'center',
    textStyle: { fontSize: 14, color: '#374151' }
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'shadow' }
  },
  grid: {
    left: '3%',
    right: '4%',
    bottom: '15%',
    containLabel: true
  },
  xAxis: {
    type: 'category',
    data: recentApps.value.map(app => app.app.length > 8 ? app.app.slice(0, 8) + '...' : app.app),
    axisLabel: {
      rotate: 30,
      fontSize: 10
    }
  },
  yAxis: {
    type: 'value'
  },
  series: [{
    type: 'bar',
    data: recentApps.value.map((app, idx) => ({
      value: app.count,
      itemStyle: { color: ['#3b82f6', '#22c55e', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#ec4899', '#84cc16'][idx % 8] }
    })),
    barWidth: '60%',
    itemStyle: {
      borderRadius: [4, 4, 0, 0]
    }
  }]
}));

onMounted(() => {
  loadStats();
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  if (resizeTimer) clearTimeout(resizeTimer);
});
</script>

<template>
  <div class="home">
    <div class="home-content">
      <div class="stats-row">
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">本周AI总结:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main placeholder">暂未实现</div>
        </div>
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">日志条目:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main">
            <span class="stat-number">{{ eventCount }}</span>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">情感趋势:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main placeholder">暂未实现</div>
        </div>
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">记录天数:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main placeholder">暂未实现</div>
        </div>
      </div>
      
      <div class="recent-apps-section">
        <div class="section-header">
          <h3>最近应用记录</h3>
        </div>
        <div class="apps-grid-full">
          <div 
            v-for="(item, idx) in recentApps" 
            :key="idx" 
            class="app-card"
            :style="{ backgroundColor: item.color }"
          >
            <div class="app-icon">
              <img v-if="item.icon" :src="item.icon" alt="" class="icon-img" />
              <span v-else class="icon-placeholder">{{ item.app.charAt(0).toUpperCase() }}</span>
            </div>
            <div class="app-info">
              <div class="app-name">{{ item.app }}</div>
              <div class="app-stats">
                <span class="app-duration">{{ formatDuration(item.totalDuration) }}</span>
                <span class="app-divider">·</span>
                <span class="app-count">{{ item.count }}次</span>
              </div>
            </div>
          </div>
          <div v-if="recentApps.length === 0" class="empty-apps">
            暂无应用记录
          </div>
        </div>
      </div>
      
      <div class="charts-section">
        <div class="section-header">
          <h3>应用行为统计</h3>
        </div>
        <div class="charts-grid">
          <div class="chart-card">
            <VChart ref="pieChartRef" :option="pieChartOption" autoresize class="chart" />
          </div>
          <div class="chart-card">
            <VChart ref="barChartRef" :option="barChartOption" autoresize class="chart" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
  overflow-y: auto;
  overflow-x: hidden;
}

.home-content {
  padding: 20px;
}

.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-card {
  background: #fff;
  border-radius: 12px;
  padding: 16px;
  border: 1px solid #e5e7eb;
}

.stat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.stat-title {
  font-size: 13px;
  color: #6b7280;
}

.stat-menu {
  color: #9ca3af;
  cursor: pointer;
}

.stat-main {
  font-size: 20px;
  font-weight: 600;
  color: #1f2937;
  display: flex;
  align-items: center;
  gap: 8px;
}

.stat-number {
  font-size: 24px;
}

.stat-badge {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 12px;
  background: #dcfce7;
  color: #16a34a;
}

.stat-badge.green {
  background: #dcfce7;
  color: #16a34a;
}

.recent-apps-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  border: 1px solid #e5e7eb;
}

.section-header {
  margin-bottom: 16px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  color: #1f2937;
}

.apps-grid-full {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

.app-card {
  border-radius: 12px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 12px;
}

.app-icon {
  width: 40px;
  height: 40px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.app-info {
  flex: 1;
  min-width: 0;
}

.app-name {
  font-size: 14px;
  font-weight: 600;
  color: #1f2937;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.app-stats {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 4px;
  font-size: 12px;
  color: #6b7280;
}

.app-duration {
  color: #374151;
  font-weight: 500;
}

.app-divider {
  color: #9ca3af;
}

.app-count {
  color: #6b7280;
}

.icon-img {
  width: 32px;
  height: 32px;
  object-fit: contain;
}

.icon-placeholder {
  width: 32px;
  height: 32px;
  background: rgba(0,0,0,0.1);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 600;
  color: #374151;
}

.stat-main.placeholder {
  color: #9ca3af;
  font-size: 14px;
  font-weight: normal;
}

.empty-apps {
  grid-column: span 4;
  text-align: center;
  padding: 40px;
  color: #9ca3af;
}

.charts-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  border: 1px solid #e5e7eb;
  margin-top: 24px;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
}

@media (max-width: 900px) {
  .charts-grid {
    grid-template-columns: 1fr;
  }
}

.chart-card {
  background: #fafafa;
  border-radius: 8px;
  padding: 16px;
  border: 1px solid #e5e7eb;
  min-height: 320px;
}

.chart {
  width: 100%;
  height: 300px;
  min-height: 280px;
}
</style>
