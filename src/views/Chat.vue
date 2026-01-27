<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { activityApi } from '@/api/activity';
import { aiApi } from '@/api/ai';

interface Note {
  id: string;
  title: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

// 获取指定日期的笔记内容
async function getNotesContentByDate(targetDate: string): Promise<string> {
  try {
    const data = await invoke<string>('load_notes');
    if (!data) return '';
    
    const notes: Note[] = JSON.parse(data);
    
    const dateNotes = notes.filter(n => {
      const noteDate = new Date(n.createdAt).toISOString().split('T')[0];
      return noteDate === targetDate;
    });
    
    if (dateNotes.length === 0) return '';
    
    return dateNotes.map(n => {
      const textContent = n.content.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
      return `【${n.title}】${textContent}`;
    }).join('\n\n');
  } catch (e) {
    console.warn('获取笔记失败:', e);
    return '';
  }
}

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: string;
}

interface ChatSession {
  id: string;
  title: string;
  date: string;
  messages: Message[];
  createdAt: string;
  updatedAt: string;
}

const messages = ref<Message[]>([]);
const inputMessage = ref('');
const isLoading = ref(false);
const hasApiKey = ref(false);
const selectedDate = ref('');
const chatContainer = ref<HTMLElement | null>(null);
const currentAssistantId = ref('');
const chatSessions = ref<ChatSession[]>([]);
const currentSessionId = ref('');

let unlistenChunk: UnlistenFn | null = null;

// 保存对话到本地
async function saveChatHistory() {
  try {
    await invoke('save_chat_history', { 
      sessions: JSON.stringify(chatSessions.value) 
    });
  } catch (e) {
    console.error('保存对话失败:', e);
  }
}

// 加载对话历史
async function loadChatHistory() {
  try {
    const data = await invoke<string>('load_chat_history');
    if (data) {
      chatSessions.value = JSON.parse(data);
      // 如果有会话，加载最新的
      if (chatSessions.value.length > 0) {
        const latest = chatSessions.value[0];
        currentSessionId.value = latest.id;
        messages.value = latest.messages;
      }
    }
  } catch (e) {
    console.warn('加载对话历史失败:', e);
  }
}

// 创建新会话
function createNewSession() {
  const session: ChatSession = {
    id: Date.now().toString(),
    title: '新对话',
    date: selectedDate.value,
    messages: [],
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  };
  chatSessions.value.unshift(session);
  currentSessionId.value = session.id;
  messages.value = [];
  saveChatHistory();
}

// 切换会话
function switchSession(sessionId: string) {
  const session = chatSessions.value.find(s => s.id === sessionId);
  if (session) {
    currentSessionId.value = sessionId;
    messages.value = session.messages;
    selectedDate.value = session.date;
    // 切换后滚动到最新消息
    scrollToBottom();
  }
}

// 更新当前会话
function updateCurrentSession() {
  const session = chatSessions.value.find(s => s.id === currentSessionId.value);
  if (session) {
    session.messages = [...messages.value];
    session.updatedAt = new Date().toISOString();
    // 用第一条用户消息作为标题
    const firstUserMsg = messages.value.find(m => m.role === 'user');
    if (firstUserMsg) {
      session.title = firstUserMsg.content.slice(0, 20) + (firstUserMsg.content.length > 20 ? '...' : '');
    }
    saveChatHistory();
  }
}

// 删除会话
function deleteSession(sessionId: string) {
  chatSessions.value = chatSessions.value.filter(s => s.id !== sessionId);
  if (currentSessionId.value === sessionId) {
    if (chatSessions.value.length > 0) {
      switchSession(chatSessions.value[0].id);
    } else {
      createNewSession();
    }
  }
  saveChatHistory();
}

// 获取今日日期
function getTodayDate() {
  const now = new Date();
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
}

// 检查API Key
async function checkApiKey() {
  try {
    const config = await aiApi.getConfig();
    hasApiKey.value = !!config.api_key;
  } catch {
    hasApiKey.value = false;
  }
}

// 获取指定日期的全部数据
async function getDateData(date: string) {
  try {
    // 获取活动数据
    let events: { app_focus: any[]; keyboard: any[]; mouse: any[]; idle: any[] } = { app_focus: [], keyboard: [], mouse: [], idle: [] };
    try {
      events = await activityApi.getGroupedEventsByDate(date);
    } catch (e) {
      console.warn('获取活动数据失败:', e);
    }

    // 获取OCR数据
    let ocrData: any[] = [];
    try {
      ocrData = await activityApi.getOcrDataByDate(date);
    } catch (e) {
      console.warn('获取OCR数据失败:', e);
    }

    // 获取截图列表
    let screenshots: string[] = [];
    try {
      screenshots = await activityApi.getScreenshotsByDate(date);
    } catch (e) {
      console.warn('获取截图列表失败:', e);
    }

    // 获取输入统计
    let inputStats = { key_count: 0, click_count: 0, mouse_distance: 0, idle_seconds: 0 };
    try {
      inputStats = await activityApi.getInputStats();
    } catch (e) {
      console.warn('获取输入统计失败:', e);
    }

    // 智能摘要：按应用聚合使用情况
    const appUsageMap = new Map<string, { count: number; titles: Set<string> }>();
    for (const event of (events.app_focus || [])) {
      const app = event.app || '未知应用';
      if (!appUsageMap.has(app)) {
        appUsageMap.set(app, { count: 0, titles: new Set() });
      }
      const usage = appUsageMap.get(app)!;
      usage.count++;
      if (event.window_title) {
        usage.titles.add(event.window_title.substring(0, 50));
      }
    }
    
    // 保留所有应用（100%）
    const appUsageSummary = Array.from(appUsageMap.entries())
      .map(([app, data]) => ({
        app,
        focus_count: data.count,
        sample_titles: Array.from(data.titles).slice(0, 3)
      }))
      .sort((a, b) => b.focus_count - a.focus_count);
    
    // OCR摘要：按比例采样（取20%，无上限），均匀采样覆盖全天
    const ocrSampleCount = Math.max(1, Math.ceil(ocrData.length * 0.2));
    const ocrStep = Math.max(1, Math.floor(ocrData.length / ocrSampleCount));
    const sampledOcr: any[] = [];
    for (let i = 0; i < ocrData.length && sampledOcr.length < ocrSampleCount; i += ocrStep) {
      sampledOcr.push(ocrData[i]);
    }
    const ocrSummary = sampledOcr.map(item => ({
      time: item.timestamp?.substring(11, 16) || '',
      app: item.app_name || '',
      text: (item.text || '').substring(0, 200)
    }));

    return {
      date,
      app_usage: appUsageSummary,
      input_summary: {
        total_keystrokes: inputStats.key_count,
        total_clicks: inputStats.click_count,
        mouse_distance_meters: Math.round(inputStats.mouse_distance / 1000),
        idle_minutes: Math.round(inputStats.idle_seconds / 60)
      },
      ocr_highlights: ocrSummary,
      statistics: {
        total_app_switches: (events.app_focus || []).length,
        unique_apps_used: appUsageMap.size,
        screenshot_count: screenshots.length,
        idle_periods: (events.idle || []).length
      }
    };
  } catch (e) {
    console.error('获取日期数据失败:', e);
    return null;
  }
}

// 滚动到底部
async function scrollToBottom() {
  await nextTick();
  if (chatContainer.value) {
    chatContainer.value.scrollTop = chatContainer.value.scrollHeight;
  }
}

// 发送消息
async function sendMessage() {
  if (!inputMessage.value.trim() || isLoading.value || !hasApiKey.value) return;

  const userMessage = inputMessage.value.trim();
  inputMessage.value = '';

  // 添加用户消息
  messages.value.push({
    id: Date.now().toString(),
    role: 'user',
    content: userMessage,
    timestamp: new Date().toLocaleTimeString()
  });
  
  // 立即保存用户消息，防止丢失
  updateCurrentSession();

  await scrollToBottom();
  isLoading.value = true;

  // 添加空的AI消息，用于流式填充
  const assistantId = (Date.now() + 1).toString();
  currentAssistantId.value = assistantId;
  messages.value.push({
    id: assistantId,
    role: 'assistant',
    content: '',
    timestamp: new Date().toLocaleTimeString()
  });

  try {
    // 获取选中日期的数据
    const dateData = await getDateData(selectedDate.value);
    
    // 获取选中日期的笔记内容
    let dateNotes = '';
    try {
      dateNotes = await getNotesContentByDate(selectedDate.value);
    } catch (e) {
      console.warn('获取笔记失败:', e);
    }
    
    // 构建系统提示
    let systemPrompt = `你是用户的AI助手，可以根据用户的电脑活动摘要数据回答问题。

当前查询日期: ${selectedDate.value}

用户的活动摘要数据:
${JSON.stringify(dateData, null, 2)}

数据说明:
- app_usage: 应用使用统计（按使用次数排序，包含应用名、焦点次数、窗口标题样本）
- input_summary: 输入统计（按键次数、点击次数、鼠标移动距离(米)、空闲分钟数）
- ocr_highlights: OCR识别的文字摘要
- statistics: 汇总统计`;

    // 如果有该日期的笔记，添加到提示中
    if (dateNotes) {
      systemPrompt += `

用户${selectedDate.value}的记事本笔记:
${dateNotes}`;
    }

    systemPrompt += `

请根据以上数据回答用户的问题。用第一人称"你"来称呼用户。回答要简洁准确。`;

    // 调用流式AI
    await aiApi.chatStream(systemPrompt, userMessage);
  } catch (e: any) {
    // 更新AI消息为错误信息
    const msg = messages.value.find(m => m.id === assistantId);
    if (msg) {
      msg.content = `抱歉，发生错误: ${e.toString()}`;
    }
    isLoading.value = false;
  }
}

// 处理回车发送
function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
}

async function setupEventListeners() {
  // 监听流式内容
  unlistenChunk = await listen<string>('chat-chunk', (event) => {
    const msg = messages.value.find(m => m.id === currentAssistantId.value);
    if (msg) {
      msg.content += event.payload;
      scrollToBottom();
    }
  });

  // 监听完成
  await listen('chat-complete', () => {
    isLoading.value = false;
    scrollToBottom();
    updateCurrentSession();
  });

  // 监听错误
  await listen<string>('chat-error', (event) => {
    const msg = messages.value.find(m => m.id === currentAssistantId.value);
    if (msg) {
      msg.content = `错误: ${event.payload}`;
    }
    isLoading.value = false;
  });
}

onMounted(async () => {
  selectedDate.value = getTodayDate();
  await checkApiKey();
  await loadChatHistory();
  // 如果没有会话，创建一个
  if (chatSessions.value.length === 0) {
    createNewSession();
  }
  await setupEventListeners();
  // 自动滚动到最新消息
  scrollToBottom();
});

onUnmounted(() => {
  unlistenChunk?.();
});
</script>

<template>
  <div class="chat-page">
    <!-- 会话列表侧边栏 -->
    <div class="sessions-sidebar">
      <div class="sidebar-header">
        <h3>对话记录</h3>
        <button class="new-chat-btn" @click="createNewSession">+ 新对话</button>
      </div>
      <div class="sessions-list">
        <div 
          v-for="session in chatSessions" 
          :key="session.id" 
          :class="['session-item', { active: session.id === currentSessionId }]"
          @click="switchSession(session.id)"
        >
          <div class="session-info">
            <span class="session-title">{{ session.title }}</span>
            <span class="session-date">{{ session.date }}</span>
          </div>
          <button class="delete-btn" @click.stop="deleteSession(session.id)">
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 6h18"></path>
              <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
              <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- 主聊天区域 -->
    <div class="chat-main">
      <div class="chat-header">
        <h2>AI 对话</h2>
        <div class="header-actions">
          <div class="date-selector">
            <label>查询日期:</label>
            <input type="date" v-model="selectedDate" />
          </div>
        </div>
      </div>

      <div class="chat-container" ref="chatContainer">
        <div v-if="messages.length === 0" class="empty-state">
          <div class="empty-icon">
            <img src="/icon.png" alt="DailyCraft" width="64" height="64" />
          </div>
          <p>选择日期，然后问我关于那天活动的任何问题</p>
          <p class="hint">例如: "我今天主要做了什么？" "我在哪个应用花的时间最多？"</p>
        </div>

        <div v-for="msg in messages" :key="msg.id" :class="['message', msg.role]">
          <div class="message-avatar">
            <svg v-if="msg.role === 'user'" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
              <circle cx="12" cy="7" r="4"></circle>
            </svg>
            <img v-else src="/icon.png" alt="AI" width="20" height="20" />
          </div>
          <div class="message-content">
            <div class="message-text">{{ msg.content }}</div>
            <div class="message-time">{{ msg.timestamp }}</div>
          </div>
        </div>

        <div v-if="isLoading && messages.length > 0 && messages[messages.length-1].content === ''" class="typing-indicator-wrapper">
          <div class="typing-indicator">
            <span></span><span></span><span></span>
          </div>
        </div>
      </div>

      <div class="chat-input-area">
        <div v-if="!hasApiKey" class="api-key-warning">
          请先在设置页面配置 API Key
        </div>
        <div class="input-wrapper">
          <textarea
            v-model="inputMessage"
            placeholder="输入你的问题..."
            @keydown="handleKeydown"
            :disabled="!hasApiKey || isLoading"
            rows="1"
          ></textarea>
          <button 
            class="send-btn" 
            @click="sendMessage"
            :disabled="!inputMessage.trim() || !hasApiKey || isLoading"
          >
            发送
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.chat-page {
  display: flex;
  flex-direction: row;
  height: calc(100vh - 60px);
  max-height: calc(100vh - 60px);
  background: #f8fafc;
  overflow: hidden;
}

/* 会话列表侧边栏 */
.sessions-sidebar {
  width: 240px;
  min-width: 240px;
  background: #fff;
  border-right: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.sidebar-header {
  padding: 16px;
  border-bottom: 1px solid #e5e7eb;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sidebar-header h3 {
  margin: 0;
  font-size: 0.95rem;
  color: #1f2937;
}

.new-chat-btn {
  padding: 6px 12px;
  border-radius: 6px;
  border: none;
  background: #6366f1;
  color: #fff;
  font-size: 0.8rem;
  cursor: pointer;
  transition: background 0.2s;
}

.new-chat-btn:hover {
  background: #4f46e5;
}

.sessions-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.session-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
  margin-bottom: 4px;
}

.session-item:hover {
  background: #f3f4f6;
}

.session-item.active {
  background: #eef2ff;
}

.session-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.session-title {
  font-size: 0.875rem;
  color: #1f2937;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.session-date {
  font-size: 0.75rem;
  color: #9ca3af;
}

.delete-btn {
  padding: 4px;
  border: none;
  background: transparent;
  color: #9ca3af;
  cursor: pointer;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.2s;
}

.session-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background: #fee2e2;
  color: #dc2626;
}

/* 主聊天区域 */
.chat-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
  overflow: hidden;
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  border-bottom: 1px solid #e5e7eb;
  background: #fff;
}

.chat-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: #1f2937;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.date-selector {
  display: flex;
  align-items: center;
  gap: 8px;
}

.date-selector label {
  color: #6b7280;
  font-size: 0.875rem;
}

.date-selector input {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid #d1d5db;
  background: #fff;
  color: #1f2937;
  font-size: 0.875rem;
}

.clear-btn {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid #d1d5db;
  background: #fff;
  color: #6b7280;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.clear-btn:hover:not(:disabled) {
  background: #f3f4f6;
  color: #1f2937;
}

.clear-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.chat-container {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 0;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
  text-align: center;
}

.empty-icon {
  margin-bottom: 16px;
  color: #d1d5db;
}

.empty-state p {
  margin: 4px 0;
  color: #6b7280;
}

.empty-state .hint {
  font-size: 0.875rem;
  color: #9ca3af;
}

.message {
  display: flex;
  gap: 12px;
  max-width: 80%;
  min-width: 0;
}

.message.user {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.message.assistant {
  align-self: flex-start;
}

.message-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.message.user .message-avatar {
  background: #6366f1;
  color: #fff;
}

.message.assistant .message-avatar {
  background: #e5e7eb;
  color: #4b5563;
}

.message-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
  max-width: 100%;
}

.message-text {
  padding: 12px 16px;
  border-radius: 16px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
  word-break: break-word;
}

.message.user .message-text {
  background: #6366f1;
  color: #fff;
  border-bottom-right-radius: 4px;
}

.message.assistant .message-text {
  background: #fff;
  color: #1f2937;
  border: 1px solid #e5e7eb;
  border-bottom-left-radius: 4px;
}

.message-time {
  font-size: 0.75rem;
  color: #9ca3af;
  padding: 0 8px;
}

.message.user .message-time {
  text-align: right;
}

.typing-indicator-wrapper {
  align-self: flex-start;
  padding: 12px 16px;
  background: #fff;
  border: 1px solid #e5e7eb;
  border-radius: 16px;
  margin-left: 48px;
}

.typing-indicator {
  display: flex;
  gap: 4px;
}

.typing-indicator span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #9ca3af;
  animation: typing 1.4s infinite ease-in-out both;
}

.typing-indicator span:nth-child(1) { animation-delay: -0.32s; }
.typing-indicator span:nth-child(2) { animation-delay: -0.16s; }

@keyframes typing {
  0%, 80%, 100% { transform: scale(0.6); opacity: 0.5; }
  40% { transform: scale(1); opacity: 1; }
}

.chat-input-area {
  padding: 16px 24px;
  border-top: 1px solid #e5e7eb;
  background: #fff;
}

.api-key-warning {
  padding: 8px 12px;
  margin-bottom: 12px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 8px;
  color: #dc2626;
  font-size: 0.875rem;
  text-align: center;
}

.input-wrapper {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}

.input-wrapper textarea {
  flex: 1;
  padding: 12px 16px;
  border-radius: 12px;
  border: 1px solid #d1d5db;
  background: #fff;
  color: #1f2937;
  font-size: 0.95rem;
  resize: none;
  min-height: 44px;
  max-height: 120px;
  font-family: inherit;
}

.input-wrapper textarea:focus {
  outline: none;
  border-color: #6366f1;
}

.input-wrapper textarea:disabled {
  opacity: 0.5;
  background: #f3f4f6;
}

.send-btn {
  padding: 12px 24px;
  border-radius: 12px;
  border: none;
  background: #6366f1;
  color: #fff;
  font-size: 0.95rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.send-btn:hover:not(:disabled) {
  background: #4f46e5;
}

.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 响应式布局 */
@media (max-width: 800px) {
  .sessions-sidebar {
    width: 180px;
    min-width: 180px;
  }
}

@media (max-width: 600px) {
  .chat-page {
    flex-direction: column;
  }
  .sessions-sidebar {
    width: 100%;
    min-width: 100%;
    height: auto;
    max-height: 200px;
    border-right: none;
    border-bottom: 1px solid #e5e7eb;
  }
  .chat-main {
    height: calc(100vh - 260px);
  }
}
</style>
