<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { activityApi } from '@/api/activity';
import { diaryApi } from '@/api/diary';
import VChart from 'vue-echarts';
import { use } from 'echarts/core';
import { CanvasRenderer } from 'echarts/renderers';
import { PieChart, BarChart, LineChart } from 'echarts/charts';
import { TitleComponent, TooltipComponent, LegendComponent, GridComponent } from 'echarts/components';

use([CanvasRenderer, PieChart, BarChart, LineChart, TitleComponent, TooltipComponent, LegendComponent, GridComponent]);

// 图表引用
const pieChartRef = ref<InstanceType<typeof VChart> | null>(null);
const barChartRef = ref<InstanceType<typeof VChart> | null>(null);

// Dashboard统计数据（前端计算）
const totalDays = ref(0);
const todayEvents = ref(0);
const todayDiary = ref<string | null>(null);

// 欢迎模块显示状态
const showWelcome = ref(true);

function dismissWelcome() {
  showWelcome.value = false;
  localStorage.setItem('dailycraft_welcome_dismissed', 'true');
}

function checkWelcomeDismissed() {
  const dismissed = localStorage.getItem('dailycraft_welcome_dismissed');
  showWelcome.value = dismissed !== 'true';
}

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
    // 加载今日事件数
    todayEvents.value = await activityApi.getTodayEventCount();
    
    // 加载总事件数
    eventCount.value = await activityApi.getTotalEventCount();
    
    // 加载日记列表获取总天数
    try {
      const diaryList = await diaryApi.getDiaryList();
      totalDays.value = diaryList.length;
      
      // 检查今日是否已生成日记
      const now = new Date();
      const today = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
      if (diaryList.includes(today)) {
        todayDiary.value = '已生成 ✓';
      }
    } catch (e) {
      console.warn('加载日记列表失败:', e);
    }
    
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
  checkWelcomeDismissed();
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
      <!-- 欢迎模块 -->
      <div v-if="showWelcome" class="welcome-section">
        <button class="welcome-close" @click="dismissWelcome" title="关闭">×</button>
        <div class="welcome-content">
          <div class="welcome-icon">
            <img src="/icon.png" alt="DailyCraft" />
          </div>
          <div class="welcome-text">
            <h2>欢迎使用 PawPrint</h2>
            <p class="welcome-desc">
              DailyCraft 是一款 AI 可视化日志分析软件，通过监控您的电脑操作，以可拖拽小卡片的形式记录每个时间段的活动数据，并在一天结束后由 AI 自动生成日记总结。
            </p>
            <p class="welcome-thanks">
              感谢您使用本软件，希望它能帮助您更好地了解自己的工作习惯和时间分配。
            </p>
          </div>
        </div>
      </div>

      <div class="stats-row">
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">AI日报:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main" :class="{ placeholder: !todayDiary }">
            {{ todayDiary || '今日未生成' }}
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">记录总天数:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main">
            <span class="stat-number">{{ totalDays }}</span>
            <span class="stat-unit">天</span>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">今日日志条目:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main">
            <span class="stat-number">{{ todayEvents }}</span>
          </div>
        </div>
        <div class="stat-card">
          <div class="stat-header">
            <span class="stat-title">总日志条目:</span>
            <span class="stat-menu">⋮</span>
          </div>
          <div class="stat-main">
            <span class="stat-number">{{ eventCount }}</span>
          </div>
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

/* 欢迎模块 */
.welcome-section {
  background: #fff;
  border-radius: 12px;
  padding: 32px 24px;
  margin-bottom: 24px;
  border: 1px solid #e5e7eb;
  text-align: center;
  position: relative;
}

.welcome-close {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 28px;
  height: 28px;
  border: none;
  background: #f3f4f6;
  border-radius: 50%;
  font-size: 18px;
  color: #6b7280;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.welcome-close:hover {
  background: #e5e7eb;
  color: #374151;
}

.welcome-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.welcome-icon {
  flex-shrink: 0;
}

.welcome-icon img {
  width: 64px;
  height: 64px;
}

.welcome-text {
  max-width: 600px;
}

.welcome-text h2 {
  margin: 0 0 12px 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #1f2937;
}

.welcome-desc {
  margin: 0 0 8px 0;
  font-size: 0.875rem;
  line-height: 1.6;
  color: #6b7280;
}

.welcome-thanks {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
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

.stat-unit {
  font-size: 14px;
  color: #6b7280;
  font-weight: normal;
  margin-left: 4px;
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
  .stats-row {
    grid-template-columns: repeat(2, 1fr);
  }
  .charts-grid {
    grid-template-columns: 1fr;
  }
  .apps-grid-full {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .stats-row {
    grid-template-columns: 1fr;
  }
  .apps-grid-full {
    grid-template-columns: 1fr;
  }
  .empty-apps {
    grid-column: span 1;
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
