<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import { openPath as _openPath } from '@tauri-apps/plugin-opener';
import { activityApi, type AppConfig } from '@/api/activity';
import { aiApi, type AIConfig } from '@/api/ai';

const dataDir = ref('');
const newDataDir = ref('');
const autoStartEnabled = ref(false);
const isLoading = ref(true);

// AI配置
const aiConfig = ref<AIConfig>({
  api_key: '',
  model: 'qwen-plus',
  base_url: 'https://dashscope.aliyuncs.com/compatible-mode/v1'
});
const showApiKey = ref(false);
const aiSaving = ref(false);

// 应用配置
const appConfig = ref<AppConfig>({
  poll_interval_ms: 1000,
  idle_threshold_sec: 300,
  screenshot_enabled: false,
  screenshot_trigger_sec: 30,
  screenshot_interval_sec: 60,
  screenshot_mode: 'full_screen',
  screenshot_hotkey: 'Alt+]'
});
const configSaving = ref(false);

// 确认弹窗
const showConfirmModal = ref(false);
const confirmModalConfig = ref({
  title: '',
  message: '',
  confirmText: '确定',
  cancelText: '取消',
  onConfirm: () => {}
});

function showConfirm(config: { title: string; message: string; onConfirm: () => void }) {
  confirmModalConfig.value = {
    title: config.title,
    message: config.message,
    confirmText: '确定',
    cancelText: '取消',
    onConfirm: config.onConfirm
  };
  showConfirmModal.value = true;
}

function closeConfirmModal() {
  showConfirmModal.value = false;
}

function handleConfirm() {
  confirmModalConfig.value.onConfirm();
  closeConfirmModal();
}

async function loadSettings() {
  isLoading.value = true;
  try {
    dataDir.value = await activityApi.getDataDir();
    newDataDir.value = dataDir.value;
    // 加载开机自启状态
    try {
      const enabled = await invoke<boolean>('plugin:autostart|is_enabled');
      console.log('开机自启状态:', enabled);
      autoStartEnabled.value = enabled;
    } catch (autoErr) {
      console.error('获取开机自启状态失败:', autoErr);
      autoStartEnabled.value = false;
    }
    // 加载AI配置
    try {
      aiConfig.value = await aiApi.getConfig();
    } catch (aiErr) {
      console.error('获取AI配置失败:', aiErr);
    }
    // 加载应用配置
    try {
      appConfig.value = await activityApi.getAppConfig();
    } catch (configErr) {
      console.error('获取应用配置失败:', configErr);
    }
  } catch (e) {
    console.error('加载设置失败:', e);
  } finally {
    isLoading.value = false;
  }
}

async function saveAiConfig() {
  aiSaving.value = true;
  try {
    await aiApi.saveConfig(aiConfig.value);
    showConfirm({
      title: '成功',
      message: 'AI配置已保存',
      onConfirm: () => {}
    });
  } catch (e) {
    console.error('保存AI配置失败:', e);
    showConfirm({
      title: '失败',
      message: '保存AI配置失败',
      onConfirm: () => {}
    });
  } finally {
    aiSaving.value = false;
  }
}

async function saveAppConfig() {
  configSaving.value = true;
  try {
    await activityApi.saveAppConfig(appConfig.value);
    // 通知App.vue重新加载配置
    await emit('config-changed', appConfig.value);
    showConfirm({
      title: '成功',
      message: '监控配置已保存并生效',
      onConfirm: () => {}
    });
  } catch (e) {
    console.error('保存应用配置失败:', e);
    showConfirm({
      title: '失败',
      message: '保存配置失败，请重试',
      onConfirm: () => {}
    });
  } finally {
    configSaving.value = false;
  }
}

async function openDataFolder() {
  try {
    await invoke('open_folder', { path: dataDir.value });
  } catch (e) {
    console.error('打开文件夹失败:', e);
    showConfirm({
      title: '错误',
      message: '打开文件夹失败',
      onConfirm: () => {}
    });
  }
}

async function clearCache() {
  showConfirm({
    title: '清理缓存',
    message: '确定要清理缓存吗？\n这将删除截图和临时文件，但保留活动记录和日记。',
    onConfirm: async () => {
      try {
        await invoke('clear_cache');
        showConfirm({
          title: '成功',
          message: '缓存已清理完成',
          onConfirm: () => {}
        });
      } catch (e) {
        console.error('清理缓存失败:', e);
        showConfirm({
          title: '失败',
          message: '清理缓存失败，请重试',
          onConfirm: () => {}
        });
      }
    }
  });
}

// 监听开机自启开关变化，自动保存（忽略初始加载）
watch(autoStartEnabled, async (enabled) => {
  if (isLoading.value) return;
  try {
    if (enabled) {
      await invoke('plugin:autostart|enable');
    } else {
      await invoke('plugin:autostart|disable');
    }
  } catch (e) {
    console.error('设置开机自启失败:', e);
  }
});

async function saveDataDir() {
  try {
    await activityApi.setDataDir(newDataDir.value);
    dataDir.value = newDataDir.value;
    showConfirm({
      title: '成功',
      message: '数据目录已更新',
      onConfirm: () => {}
    });
  } catch (e) {
    console.error('保存失败:', e);
    showConfirm({
      title: '失败',
      message: '保存数据目录失败',
      onConfirm: () => {}
    });
  }
}

async function selectDirectory() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择数据存储目录'
    });
    if (selected && typeof selected === 'string') {
      newDataDir.value = selected;
    }
  } catch (e) {
    console.error('选择目录失败:', e);
  }
}

onMounted(() => {
  loadSettings();
});
</script>

<template>
  <div class="settings">
    <div class="settings-content">
      <div class="setting-item">
        <label class="setting-label">数据存储目录</label>
        <div class="setting-input-group">
          <input 
            v-model="newDataDir" 
            type="text" 
            class="setting-input"
            placeholder="输入数据存储路径"
            readonly
          />
          <button class="animated-button small" @click="selectDirectory">
            <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
              <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"></path>
            </svg>
            <span class="text">浏览</span>
            <span class="circle"></span>
            <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
              <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"></path>
            </svg>
          </button>
          <button class="animated-button small primary" @click="saveDataDir">
            <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
            </svg>
            <span class="text">保存</span>
            <span class="circle"></span>
            <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
            </svg>
          </button>
        </div>
        <p class="setting-hint">当前: {{ dataDir }}</p>
      </div>
      
      <div class="setting-item">
        <label class="setting-label">启动设置</label>
        <div class="setting-option">
          <input type="checkbox" id="autoLaunch" v-model="autoStartEnabled" />
          <label for="autoLaunch">开机自动启动</label>
        </div>
      </div>
      
      <div class="setting-item">
        <label class="setting-label">监控设置</label>
        <div class="setting-row">
          <span class="setting-row-label">检查间隔（毫秒）</span>
          <input 
            v-model.number="appConfig.poll_interval_ms" 
            type="number" 
            class="setting-input-small"
            min="100"
            max="10000"
            step="100"
          />
          <span class="setting-row-hint">数值越小性能消耗越大，默认1000</span>
        </div>
        <div class="setting-row">
          <span class="setting-row-label">空闲判定阈值（秒）</span>
          <input 
            v-model.number="appConfig.idle_threshold_sec" 
            type="number" 
            class="setting-input-small"
            min="60"
            max="3600"
            step="60"
          />
          <span class="setting-row-hint">超过此时间判定为空闲，默认300</span>
        </div>
        
        <div class="setting-divider"></div>
        <label class="setting-sublabel">截图设置</label>
        <div class="setting-option">
          <input type="checkbox" id="screenshotEnabled" v-model="appConfig.screenshot_enabled" />
          <label for="screenshotEnabled">启用自动截图</label>
        </div>
        <div v-if="appConfig.screenshot_enabled" class="screenshot-options">
          <div class="setting-row">
            <span class="setting-row-label">触发截图（秒）</span>
            <input 
              v-model.number="appConfig.screenshot_trigger_sec" 
              type="number" 
              class="setting-input-small"
              min="5"
              max="300"
            />
            <span class="setting-row-hint">在应用停留多少秒后自动截图</span>
          </div>
          <div class="setting-row">
            <span class="setting-row-label">截图间隔（秒）</span>
            <input 
              v-model.number="appConfig.screenshot_interval_sec" 
              type="number" 
              class="setting-input-small"
              min="10"
              max="600"
            />
            <span class="setting-row-hint">首次截图后每隔多少秒再次截图</span>
          </div>
          <div class="setting-row">
            <span class="setting-row-label">截图模式</span>
            <select v-model="appConfig.screenshot_mode" class="setting-select">
              <option value="full_screen">全屏截图</option>
              <option value="app_window">应用窗口</option>
            </select>
          </div>
        </div>
        <div class="setting-row">
          <span class="setting-row-label">手动截图快捷键</span>
          <input 
            v-model="appConfig.screenshot_hotkey" 
            type="text" 
            class="setting-input-small"
            placeholder="如 Alt+]"
          />
        </div>
        <button class="animated-button primary config-save-btn" @click="saveAppConfig" :disabled="configSaving">
          <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
            <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"></path>
          </svg>
          <span class="text">{{ configSaving ? '保存中...' : '保存监控设置' }}</span>
          <span class="circle"></span>
          <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
            <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"></path>
          </svg>
        </button>
      </div>
      
      <div class="setting-item">
        <label class="setting-label">数据管理</label>
        <div class="setting-btn-group">
          <button class="animated-button" @click="openDataFolder">
            <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
              <path d="M10 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"></path>
            </svg>
            <span class="text">打开数据文件夹</span>
            <span class="circle"></span>
            <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
              <path d="M10 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"></path>
            </svg>
          </button>
          <button class="animated-button danger" @click="clearCache">
            <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
              <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"></path>
            </svg>
            <span class="text">清理缓存</span>
            <span class="circle"></span>
            <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
              <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"></path>
            </svg>
          </button>
        </div>
      </div>
      
      <div class="setting-item">
        <label class="setting-label">AI日记配置（阿里百炼）</label>
        <div class="setting-input-group">
          <input 
            v-model="aiConfig.api_key" 
            :type="showApiKey ? 'text' : 'password'" 
            class="setting-input"
            placeholder="输入API Key"
          />
          <button class="animated-button small" @click="showApiKey = !showApiKey">
            <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"></path>
            </svg>
            <span class="text">{{ showApiKey ? '隐藏' : '显示' }}</span>
            <span class="circle"></span>
            <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"></path>
            </svg>
          </button>
          <button class="animated-button small primary" @click="saveAiConfig" :disabled="aiSaving">
            <svg viewBox="0 0 24 24" class="arr-2" xmlns="http://www.w3.org/2000/svg">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
            </svg>
            <span class="text">{{ aiSaving ? '保存中...' : '保存' }}</span>
            <span class="circle"></span>
            <svg viewBox="0 0 24 24" class="arr-1" xmlns="http://www.w3.org/2000/svg">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
            </svg>
          </button>
        </div>
        <p class="setting-hint">模型: {{ aiConfig.model }} | <a href="https://bailian.console.aliyun.com/" target="_blank">获取API Key</a></p>
      </div>
    </div>

    <!-- 确认弹窗 -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showConfirmModal" class="modal-overlay" @click.self="closeConfirmModal">
          <div class="modal-container">
            <div class="modal-header">
              <h3>{{ confirmModalConfig.title }}</h3>
            </div>
            <div class="modal-body">
              <p>{{ confirmModalConfig.message }}</p>
            </div>
            <div class="modal-footer">
              <button class="modal-btn cancel" @click="closeConfirmModal">{{ confirmModalConfig.cancelText }}</button>
              <button class="modal-btn confirm" @click="handleConfirm">{{ confirmModalConfig.confirmText }}</button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
  overflow: auto;
}

.settings-content {
  padding: 24px;
  min-width: 800px;
  margin: 0 auto;
}

.setting-item {
  position: relative;
  margin-bottom: 24px;
  padding: 20px;
  background: #f0f0f0;
  border: 4px solid #000;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
  transform-style: preserve-3d;
  box-shadow: 6px 6px 0 #000;
}

.setting-item:hover {
  transform: translate(-2px, -2px);
  box-shadow: 10px 10px 0 -2px #e9b50b, 10px 10px 0 0 #000;
}

.setting-label {
  display: block;
  font-size: 14px;
  font-weight: 700;
  color: #000;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 3px solid #000;
}

.setting-input-group {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.setting-input {
  flex: 1;
  min-width: 200px;
  padding: 12px 15px;
  background: #fff;
  border: 3px solid #000;
  color: #000;
  font-size: 14px;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
}

.setting-input:focus {
  outline: none;
  box-shadow: 4px 4px 0 #000;
  transform: translate(-2px, -2px);
}

.setting-hint {
  margin-top: 10px;
  font-size: 12px;
  color: #666;
}

.setting-hint a {
  color: #3b82f6;
  text-decoration: none;
  font-weight: 600;
}

.setting-hint a:hover {
  text-decoration: underline;
}

.setting-option {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 12px;
  color: #000;
  font-size: 14px;
  font-weight: 500;
}

.setting-option input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: #e9b50b;
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
  flex-wrap: wrap;
}

.setting-row-label {
  min-width: 140px;
  font-size: 14px;
  color: #000;
  font-weight: 500;
}

.setting-row-hint {
  font-size: 12px;
  color: #666;
}

.setting-input-small {
  width: 100px;
  padding: 10px 12px;
  background: #fff;
  border: 3px solid #000;
  color: #000;
  font-size: 14px;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
}

.setting-input-small:focus {
  outline: none;
  box-shadow: 4px 4px 0 #000;
  transform: translate(-2px, -2px);
}

.setting-select {
  padding: 10px 12px;
  background: #fff;
  border: 3px solid #000;
  color: #000;
  font-size: 14px;
  cursor: pointer;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
}

.setting-select:focus {
  outline: none;
  box-shadow: 4px 4px 0 #000;
}

.screenshot-options {
  padding-left: 20px;
  border-left: 4px solid #e9b50b;
  margin-left: 8px;
  margin-top: 12px;
}

.config-save-btn {
  margin-top: 16px;
}

.setting-divider {
  height: 3px;
  background: #000;
  margin: 16px 0;
}

.setting-sublabel {
  display: block;
  font-size: 13px;
  font-weight: 600;
  color: #000;
  margin-bottom: 8px;
}

.setting-btn-group {
  display: flex;
  gap: 12px;
  margin-top: 8px;
  flex-wrap: wrap;
}

/* 动画按钮样式 */
.animated-button {
  position: relative;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 12px 24px;
  border: 4px solid;
  border-color: transparent;
  font-size: 14px;
  background-color: inherit;
  border-radius: 100px;
  font-weight: 600;
  color: #3b82f6;
  box-shadow: 0 0 0 2px #3b82f6;
  cursor: pointer;
  overflow: hidden;
  transition: all 0.6s cubic-bezier(0.23, 1, 0.32, 1);
}

.animated-button.small {
  padding: 8px 16px;
  font-size: 13px;
}

.animated-button.primary {
  color: #e9b50b;
  box-shadow: 0 0 0 2px #e9b50b;
}

.animated-button.danger {
  color: #dc2626;
  box-shadow: 0 0 0 2px #dc2626;
}

.animated-button svg {
  position: absolute;
  width: 20px;
  fill: currentColor;
  z-index: 9;
  transition: all 0.8s cubic-bezier(0.23, 1, 0.32, 1);
}

.animated-button.small svg {
  width: 16px;
}

.animated-button .arr-1 {
  right: 12px;
}

.animated-button .arr-2 {
  left: -25%;
}

.animated-button .circle {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 20px;
  height: 20px;
  background-color: #3b82f6;
  border-radius: 50%;
  opacity: 0;
  transition: all 0.8s cubic-bezier(0.23, 1, 0.32, 1);
}

.animated-button.primary .circle {
  background-color: #e9b50b;
}

.animated-button.danger .circle {
  background-color: #dc2626;
}

.animated-button .text {
  position: relative;
  z-index: 1;
  transform: translateX(-8px);
  transition: all 0.8s cubic-bezier(0.23, 1, 0.32, 1);
}

.animated-button:hover {
  box-shadow: 0 0 0 12px transparent;
  color: #fff;
  border-radius: 12px;
}

.animated-button.primary:hover,
.animated-button.danger:hover {
  color: #000;
}

.animated-button:hover .arr-1 {
  right: -25%;
}

.animated-button:hover .arr-2 {
  left: 12px;
}

.animated-button:hover .text {
  transform: translateX(8px);
}

.animated-button:hover svg {
  fill: #fff;
}

.animated-button.primary:hover svg,
.animated-button.danger:hover svg {
  fill: #000;
}

.animated-button:active {
  scale: 0.95;
  box-shadow: 0 0 0 4px currentColor;
}

.animated-button:hover .circle {
  width: 220px;
  height: 220px;
  opacity: 1;
}

.animated-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.animated-button:disabled:hover {
  box-shadow: 0 0 0 2px currentColor;
  color: currentColor;
}

.animated-button:disabled:hover .circle {
  width: 20px;
  height: 20px;
  opacity: 0;
}

</style>

<style>
/* 弹窗样式 - 全局样式因为Teleport传送到body */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  backdrop-filter: blur(4px);
}

.modal-container {
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  min-width: 320px;
  max-width: 400px;
  overflow: hidden;
  animation: modal-bounce 0.3s ease-out;
}

@keyframes modal-bounce {
  0% {
    transform: scale(0.8) translateY(-20px);
    opacity: 0;
  }
  50% {
    transform: scale(1.02);
  }
  100% {
    transform: scale(1) translateY(0);
    opacity: 1;
  }
}

.modal-header {
  padding: 20px 24px 12px;
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.modal-body {
  padding: 0 24px 20px;
}

.modal-body p {
  margin: 0;
  font-size: 14px;
  color: #6b7280;
  line-height: 1.6;
  white-space: pre-line;
}

.modal-footer {
  display: flex;
  gap: 12px;
  padding: 16px 24px;
  background: #f9fafb;
  border-top: 1px solid #e5e7eb;
}

.modal-btn {
  flex: 1;
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.modal-btn.cancel {
  background: #f3f4f6;
  color: #374151;
}

.modal-btn.cancel:hover {
  background: #e5e7eb;
}

.modal-btn.confirm {
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  color: white;
}

.modal-btn.confirm:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
}

.modal-btn.confirm:active {
  transform: translateY(0);
}

/* 弹窗过渡动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: scale(0.9);
}
</style>
