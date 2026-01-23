<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { marked } from 'marked';
import { activityApi } from '@/api/activity';
import { aiApi } from '@/api/ai';

const isGenerating = ref(false);
const diary = ref('');
const error = ref('');
const hasApiKey = ref(false);
const diaryList = ref<string[]>([]);
const selectedDate = ref('');

// 渲染Markdown
const renderedDiary = computed(() => {
  if (!diary.value) return '';
  return marked(diary.value) as string;
});

let unlistenChunk: UnlistenFn | null = null;
let unlistenComplete: UnlistenFn | null = null;
let unlistenError: UnlistenFn | null = null;

const defaultPrompt = `你是一个日记助手，请根据以下用户的电脑活动记录和屏幕OCR文字内容，帮我生成一篇简洁、有条理的日记。

数据说明：
- activities: 应用使用记录，包括打开的应用、窗口标题等
- ocr_texts: 截图中识别出的文字内容，可以了解用户具体在做什么

要求：
1. 用第一人称"我"来写
2. 结合活动记录和OCR文字内容，总结主要的工作和活动
3. 按时间顺序组织内容
4. 语言自然流畅，像真实的日记
5. 适当添加对工作效率的反思
6. 字数控制在300-500字
7. 使用Markdown格式输出，包括标题、列表、加粗等

输出格式示例：
# 今日日记

## 工作内容
- **项目A**: 完成了xxx
- **项目B**: 进行了yyy

## 反思
今天效率...`;

async function checkApiKey() {
  try {
    const config = await aiApi.getConfig();
    hasApiKey.value = !!config.api_key;
  } catch {
    hasApiKey.value = false;
  }
}

async function loadDiaryList() {
  try {
    diaryList.value = await aiApi.getDiaryList();
  } catch (e) {
    console.error('加载日记列表失败:', e);
  }
}

async function setupEventListeners() {
  // 监听流式内容
  unlistenChunk = await listen<string>('diary-chunk', (event) => {
    diary.value += event.payload;
  });
  
  // 监听生成完成
  unlistenComplete = await listen<string>('diary-complete', async () => {
    isGenerating.value = false;
    await loadDiaryList();
  });
  
  // 监听错误
  unlistenError = await listen<string>('diary-error', (event) => {
    error.value = event.payload;
    isGenerating.value = false;
  });
}

function cleanupEventListeners() {
  unlistenChunk?.();
  unlistenComplete?.();
  unlistenError?.();
}

async function startGeneration() {
  if (!hasApiKey.value) {
    error.value = '请先在设置页面配置API Key';
    return;
  }
  
  error.value = '';
  diary.value = '';
  const now = new Date();
  const today = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
  selectedDate.value = today;
  
  try {
    const events = await activityApi.getGroupedEvents();
    
    // 获取今日OCR数据
    let ocrData: any[] = [];
    try {
      ocrData = await activityApi.getOcrDataByDate(today);
    } catch (e) {
      console.warn('获取OCR数据失败:', e);
    }
    
    // 组合活动数据和OCR数据
    const combinedData = {
      activities: events,
      ocr_texts: ocrData.map(item => ({
        time: item.timestamp,
        app: item.app_name,
        content: item.text
      }))
    };
    
    const activitiesJson = JSON.stringify(combinedData, null, 2);
    isGenerating.value = true;
    await aiApi.startDiaryGeneration(activitiesJson, defaultPrompt);
  } catch (e: any) {
    error.value = e.toString();
    isGenerating.value = false;
  }
}

async function viewDiary(date: string) {
  try {
    const content = await aiApi.readDiary(date);
    // 提取markdown内容（去掉标题和footer）
    const lines = content.split('\n');
    const contentLines = lines.slice(2, -3); // 跳过标题和footer
    diary.value = contentLines.join('\n').trim();
    selectedDate.value = date;
    error.value = '';
  } catch (e: any) {
    error.value = e.toString();
  }
}

onMounted(async () => {
  await checkApiKey();
  await loadDiaryList();
  await setupEventListeners();
  
  // 检查是否有正在生成的日记
  const generating = await aiApi.isDiaryGenerating();
  if (generating) {
    isGenerating.value = true;
    // 获取当前状态
    const state = await aiApi.getDiaryState();
    if (state.content) {
      diary.value = state.content;
      selectedDate.value = state.date;
    }
  }
});

onUnmounted(() => {
  cleanupEventListeners();
});
</script>

<template>
  <div class="diary">
    <div class="diary-content">
      <div class="diary-actions">
        <button 
          class="generate-btn" 
          @click="startGeneration"
          :disabled="isGenerating || !hasApiKey"
        >
          <div class="svg-wrapper-1">
            <div class="svg-wrapper">
              <svg v-if="!isGenerating" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
                <path fill="none" d="M0 0h24v24H0z"></path>
                <path fill="currentColor" d="M1.946 9.315c-.522-.174-.527-.455.01-.634l19.087-6.362c.529-.176.832.12.684.638l-5.454 19.086c-.15.529-.455.547-.679.045L12 14l6-8-8 6-8.054-2.685z"></path>
              </svg>
              <span v-else class="loading-spinner"></span>
            </div>
          </div>
          <span class="btn-text">{{ isGenerating ? '生成中...' : '生成今日日记' }}</span>
          <div class="hover-hints">
            <div class="hint-left">
              <span class="hint-line"></span>
              <span class="hint-text">AI日记生成</span>
            </div>
            <div class="hint-right">
              <span class="hint-line"></span>
              <span class="hint-text">根据活动记录</span>
            </div>
          </div>
        </button>
        
        <p v-if="!hasApiKey" class="no-key-hint">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
          </svg>
          请先在设置页面配置AI API Key
        </p>
      </div>
      
      <div v-if="error" class="diary-error">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="15" y1="9" x2="9" y2="15"></line>
          <line x1="9" y1="9" x2="15" y2="15"></line>
        </svg>
        {{ error }}
      </div>
      
      <div v-if="diaryList.length > 0" class="diary-history">
        <h3 class="history-title">历史日记</h3>
        <div class="history-list">
          <button 
            v-for="date in diaryList" 
            :key="date"
            class="history-item"
            :class="{ active: selectedDate === date }"
            @click="viewDiary(date)"
          >
            {{ date }}
          </button>
        </div>
      </div>
      
      <div v-if="diary" class="diary-result">
        <div class="diary-paper">
          <div class="paper-spine"></div>
          <div class="paper-main">
            <div class="paper-header">
              <div class="header-left">
                <svg class="paper-icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#8b4513" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                  <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                </svg>
                <span class="paper-title">我的日记</span>
              </div>
              <div class="header-right">
                <span class="paper-weekday">{{ new Date(selectedDate || new Date()).toLocaleDateString('zh-CN', { weekday: 'long' }) }}</span>
                <span class="paper-date">{{ selectedDate || new Date().toISOString().split('T')[0] }}</span>
              </div>
            </div>
            <div class="paper-divider"></div>
            <div class="paper-content markdown-body" v-html="renderedDiary"></div>
            <div class="paper-footer">
              <span class="footer-text">由 AI 根据活动记录生成</span>
            </div>
          </div>
        </div>
      </div>
      
      <div v-if="!diary && !isGenerating && !error" class="diary-empty">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
          <line x1="16" y1="13" x2="8" y2="13"></line>
          <line x1="16" y1="17" x2="8" y2="17"></line>
          <polyline points="10 9 9 9 8 9"></polyline>
        </svg>
        <p>点击上方按钮生成今日日记</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.diary {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
  overflow: auto;
}

.diary-content {
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
  width: 100%;
}

.diary-actions {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  margin-bottom: 0px;
  padding: 30px 0;
}

.generate-btn {
  font-family: inherit;
  font-size: 18px;
  background: royalblue;
  color: white;
  padding: 0.7em 1em;
  padding-left: 0.9em;
  display: flex;
  align-items: center;
  border: none;
  border-radius: 16px;
  overflow: visible;
  transition: all 0.2s;
  cursor: pointer;
  position: relative;
}

.generate-btn .btn-text {
  display: block;
  margin-left: 0.3em;
  transition: all 0.3s ease-in-out;
}

.generate-btn svg {
  display: block;
  transform-origin: center center;
  transition: transform 0.3s ease-in-out;
}

.generate-btn:hover .svg-wrapper {
  animation: fly-1 0.6s ease-in-out infinite alternate;
}

.generate-btn:hover svg {
  transform: rotate(45deg) scale(1.1);
}

.generate-btn:hover .btn-text {
  transform: translateX(0.5em);
}

.generate-btn:active {
  transform: scale(0.95);
}

.generate-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.generate-btn:disabled:hover .svg-wrapper {
  animation: none;
}

.generate-btn:disabled:hover svg {
  transform: none;
}

.generate-btn:disabled:hover .btn-text {
  transform: none;
}

@keyframes fly-1 {
  from { transform: translateY(0.1em); }
  to { transform: translateY(-0.1em); }
}

.hover-hints {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.generate-btn:hover .hover-hints {
  opacity: 1;
}

.generate-btn:disabled:hover .hover-hints {
  opacity: 0;
}

.hint-left, .hint-right {
  position: absolute;
  top: 50%;
  display: flex;
  align-items: center;
}

.hint-left {
  right: 100%;
  transform: translateY(-50%);
  margin-right: 10px;
}

.hint-right {
  left: 100%;
  transform: translateY(-50%);
  margin-left: 10px;
}

.hint-line {
  width: 30px;
  height: 2px;
  background: #333;
}

.hint-left .hint-line { margin-right: 8px; }
.hint-right .hint-line { margin-left: 8px; order: 1; }

.hint-text {
  color: #333;
  font-size: 12px;
  white-space: nowrap;
  font-weight: 500;
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border: 2px solid #fff;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

.no-key-hint {
  display: flex;
  align-items: center;
  gap: 6px;
  color: #6b7280;
  font-size: 13px;
  margin: 0;
}

.diary-error {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 16px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 8px;
  color: #dc2626;
  margin-bottom: 24px;
}

.diary-result { animation: fadeIn 0.5s ease; }
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.diary-paper {
  display: flex;
  background: linear-gradient(135deg, #fdfbf7 0%, #f5f0e8 100%);
  border-radius: 4px 16px 16px 4px;
  box-shadow: 
    0 10px 40px rgba(0, 0, 0, 0.15),
    0 0 0 1px rgba(0, 0, 0, 0.05),
    inset 0 0 80px rgba(255, 255, 255, 0.5);
  overflow: hidden;
  position: relative;
}

.paper-spine {
  width: 24px;
  background: linear-gradient(90deg, #8b4513 0%, #a0522d 50%, #8b4513 100%);
  position: relative;
  flex-shrink: 0;
}

.paper-spine::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: repeating-linear-gradient(
    0deg,
    transparent,
    transparent 30px,
    rgba(0,0,0,0.1) 30px,
    rgba(0,0,0,0.1) 32px
  );
}

.paper-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: linear-gradient(
    to bottom,
    transparent 0px,
    transparent 31px,
    #e8e4dc 31px,
    #e8e4dc 32px
  );
  background-size: 100% 32px;
  padding-left: 10px;
  border-left: 2px solid #dc5c5c;
}

.paper-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px 16px 8px;
  background: linear-gradient(135deg, #fdfbf7 0%, #f5f0e8 100%);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.paper-icon { font-size: 24px; }

.paper-title {
  font-size: 20px;
  font-weight: 700;
  color: #2c1810;
  font-family: 'Georgia', serif;
}

.header-right {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
}

.paper-weekday {
  font-size: 12px;
  color: #8b7355;
}

.paper-date {
  font-size: 16px;
  color: #5c4033;
  font-weight: 600;
  font-family: 'Georgia', serif;
}

.paper-divider {
  height: 2px;
  background: linear-gradient(90deg, #dc5c5c 0%, #e8a87c 50%, #dc5c5c 100%);
  margin: 0 24px 0 8px;
  opacity: 0.6;
}

.paper-content {
  padding: 24px 24px 24px 8px;
  font-size: 15px;
  line-height: 2;
  color: #2c1810;
  min-height: 200px;
  flex: 1;
  font-family: 'Georgia', 'Noto Serif SC', serif;
}

.paper-footer {
  padding: 16px 24px 20px 8px;
  text-align: right;
}

.footer-text {
  font-size: 12px;
  color: #a89078;
  font-style: italic;
}

.paper-content.markdown-body :deep(h1) { font-size: 1.4em; margin: 0 0 16px 0; font-weight: 700; color: #2c1810; font-family: 'Georgia', serif; }
.paper-content.markdown-body :deep(h2) { font-size: 1.2em; margin: 16px 0 12px 0; font-weight: 600; color: #3d2817; font-family: 'Georgia', serif; }
.paper-content.markdown-body :deep(h3) { font-size: 1.05em; margin: 12px 0 8px 0; font-weight: 600; color: #5c4033; }
.paper-content.markdown-body :deep(p) { margin: 8px 0; }
.paper-content.markdown-body :deep(ul), .paper-content.markdown-body :deep(ol) { margin: 8px 0; padding-left: 24px; }
.paper-content.markdown-body :deep(li) { margin: 4px 0; }
.paper-content.markdown-body :deep(strong) { font-weight: 600; color: #2c1810; }
.paper-content.markdown-body :deep(em) { font-style: italic; color: #5c4033; }
.paper-content.markdown-body :deep(code) { background: rgba(139,69,19,0.1); padding: 2px 6px; border-radius: 4px; font-size: 0.9em; color: #5c4033; }
.paper-content.markdown-body :deep(blockquote) { border-left: 3px solid #dc5c5c; padding-left: 12px; margin: 12px 0; color: #8b7355; font-style: italic; }

.diary-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: #9ca3af;
  text-align: center;
}
.diary-empty svg { margin-bottom: 16px; opacity: 0.5; }
.diary-empty p { margin: 0; font-size: 14px; }

.diary-history {
  margin-top: 0px;
  margin-bottom: 20px;
  padding: 20px;
  background: #fff;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
}

.history-title {
  color: #1f2937;
  font-size: 16px;
  margin: 0 0 16px 0;
  font-weight: 600;
}

.history-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.history-item {
  padding: 8px 16px;
  background: #f3f4f6;
  border: 1px solid #e5e7eb;
  border-radius: 20px;
  color: #374151;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
}
.history-item:hover { background: #e5e7eb; }
.history-item.active {
  background: #3b82f6;
  border-color: #3b82f6;
  color: white;
}
</style>
