<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { activityApi } from '@/api/activity';

const dataDir = ref('');
const newDataDir = ref('');

async function loadSettings() {
  try {
    dataDir.value = await activityApi.getDataDir();
    newDataDir.value = dataDir.value;
  } catch (e) {
    console.error('加载设置失败:', e);
  }
}

async function saveDataDir() {
  try {
    await activityApi.setDataDir(newDataDir.value);
    dataDir.value = newDataDir.value;
    alert('数据目录已更新');
  } catch (e) {
    console.error('保存失败:', e);
    alert('保存失败');
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
          <button class="browse-btn" @click="selectDirectory">浏览...</button>
          <button class="save-btn" @click="saveDataDir">保存</button>
        </div>
        <p class="setting-hint">当前: {{ dataDir }}</p>
      </div>
      
      <div class="setting-item">
        <label class="setting-label">监控设置</label>
        <div class="setting-option">
          <input type="checkbox" id="autoStart" checked />
          <label for="autoStart">启动时自动开始监控</label>
        </div>
        <div class="setting-option">
          <input type="checkbox" id="screenshot" />
          <label for="screenshot">启用截图功能</label>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
}

.settings-content {
  padding: 24px;
}

.setting-item {
  margin-bottom: 24px;
  padding: 20px;
  background: #fff;
  border-radius: 8px;
  border: 1px solid #e5e7eb;
}

.setting-label {
  display: block;
  font-size: 14px;
  font-weight: 600;
  color: #1f2937;
  margin-bottom: 12px;
}

.setting-input-group {
  display: flex;
  gap: 12px;
}

.setting-input {
  flex: 1;
  padding: 10px 14px;
  background: #f9fafb;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  color: #1f2937;
  font-size: 14px;
}

.setting-input:focus {
  outline: none;
  border-color: #3b82f6;
  background: #fff;
}

.browse-btn {
  padding: 10px 16px;
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  color: #374151;
  font-size: 14px;
  cursor: pointer;
}

.browse-btn:hover {
  background: #e5e7eb;
}

.save-btn {
  padding: 10px 20px;
  background: #3b82f6;
  border: none;
  border-radius: 6px;
  color: white;
  font-size: 14px;
  cursor: pointer;
}

.save-btn:hover {
  background: #2563eb;
}

.setting-hint {
  margin-top: 8px;
  font-size: 12px;
  color: #6b7280;
}

.setting-option {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
  color: #9ca3af;
  font-size: 14px;
}

.setting-option input[type="checkbox"] {
  width: 16px;
  height: 16px;
}
</style>
