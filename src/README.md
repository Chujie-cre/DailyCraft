# DailyCraft 前端架构设计

## 技术栈

- **Vue 3** + Composition API
- **TypeScript** 类型安全
- **Vite** 构建工具
- **VueFlow** 活动流程图
- **Tauri API** 与后端通信

## 目录结构

```
src/
├── main.ts              # 入口文件
├── App.vue              # 根组件（页面切换、截图监听、快捷键）
├── vite-env.d.ts        # Vite类型声明
│
├── assets/              # 静态资源
│   └── main.css         # 全局样式
│
├── components/          # 通用组件
│   ├── common/          # 基础UI组件
│   │   └── ConfirmModal.vue  # 确认弹窗
│   ├── layout/          # 布局组件
│   │   ├── Header.vue   # 顶部导航（日期显示、追踪控制）
│   │   └── Sidebar.vue  # 侧边栏导航
│   └── flow/            # 流程图组件
│       └── ActivityFlow.vue  # 活动流程图（VueFlow）
│
├── views/               # 页面视图
│   ├── Home.vue         # 首页（统计概览、快速操作）
│   ├── Dashboard.vue    # 日志页（时间线、活动列表）
│   ├── Screenshots.vue  # 截图页（时间线浏览、OCR查看）
│   ├── Diary.vue        # 日记页（AI生成、Markdown渲染）
│   ├── Settings.vue     # 设置页（存储、截图、AI配置）
│   └── About.vue        # 关于页
│
├── api/                 # Tauri命令封装
│   └── activity.ts      # 活动/截图/OCR相关API
│
└── types/               # TypeScript类型
    └── flow.ts          # VueFlow节点类型
```

## 模块职责

### components/ - 组件层

| 目录 | 职责 |
|------|------|
| `common/` | 基础UI组件，可复用于任何页面 |
| `layout/` | 布局组件，Header、Sidebar等 |
| `activity/` | 活动相关的业务组件 |

### views/ - 页面视图

| 文件 | 职责 |
|------|------|
| `Dashboard.vue` | 首页仪表盘，展示今日活动概览 |
| `Timeline.vue` | 时间线视图，按时间展示活动 |
| `Diary.vue` | AI生成的日记视图 |
| `Settings.vue` | 应用设置页面 |
| `PluginMarket.vue` | 插件市场，浏览/安装插件 |

### composables/ - 组合式函数

```typescript
// useActivity.ts 示例
export function useActivity() {
  const activities = ref<Activity[]>([]);
  const loading = ref(false);
  
  async function fetchToday() {
    loading.value = true;
    activities.value = await invoke('get_today_activities');
    loading.value = false;
  }
  
  return { activities, loading, fetchToday };
}
```

### stores/ - Pinia状态管理

```typescript
// stores/activity.ts 示例
export const useActivityStore = defineStore('activity', () => {
  const activities = ref<Activity[]>([]);
  const currentActivity = ref<Activity | null>(null);
  
  async function startTracking() {
    await invoke('start_tracking');
  }
  
  async function stopTracking() {
    await invoke('stop_tracking');
  }
  
  return { activities, currentActivity, startTracking, stopTracking };
});
```

### api/ - Tauri命令封装

```typescript
// api/activity.ts 示例
import { invoke } from '@tauri-apps/api/core';
import type { Activity } from '@/types';

export async function getTodayActivities(): Promise<Activity[]> {
  return invoke('get_today_activities');
}

export async function saveActivity(activity: Activity): Promise<void> {
  return invoke('save_activity', { activity });
}

export async function generateDiary(date: string): Promise<string> {
  return invoke('generate_diary', { date });
}
```

### types/ - TypeScript类型

```typescript
// types/activity.ts 示例
export interface Activity {
  id: string;
  appName: string;
  windowTitle: string;
  startTime: string;
  endTime: string;
  duration: number;
}

export interface Card {
  id: string;
  activityId: string;
  position: { x: number; y: number };
  color: string;
  tags: string[];
}
```

## 扩展性设计

### 插件系统前端支持

```typescript
// types/plugin.ts
export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  components?: string[];  // 自定义组件
  hooks?: string[];       // 支持的钩子
}

// composables/usePlugin.ts
export function usePlugin() {
  async function installPlugin(pluginId: string) {
    await invoke('install_plugin', { pluginId });
  }
  
  async function listPlugins(): Promise<PluginManifest[]> {
    return invoke('list_plugins');
  }
  
  return { installPlugin, listPlugins };
}
```

### 用户认证扩展

```typescript
// stores/user.ts
export const useUserStore = defineStore('user', () => {
  const user = ref<User | null>(null);
  const isLoggedIn = computed(() => !!user.value);
  
  async function login(email: string, password: string) {
    user.value = await invoke('login', { email, password });
  }
  
  async function logout() {
    await invoke('logout');
    user.value = null;
  }
  
  return { user, isLoggedIn, login, logout };
});
```

## 依赖规划

```json
{
  "dependencies": {
    "vue": "^3.4",
    "@tauri-apps/api": "^2.0",
    "pinia": "^2.1",
    "vue-router": "^4.2",
    "@vueuse/core": "^10.0"
  },
  "devDependencies": {
    "tailwindcss": "^3.4",
    "typescript": "^5.0",
    "vite": "^5.0",
    "@vitejs/plugin-vue": "^5.0"
  }
}
```

## 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 组件 | PascalCase | `ActivityCard.vue` |
| 组合式函数 | camelCase + use前缀 | `useActivity.ts` |
| Store | camelCase + use前缀 + Store后缀 | `useActivityStore` |
| 类型 | PascalCase | `Activity`, `CardData` |
| 工具函数 | camelCase | `formatDate()` |
