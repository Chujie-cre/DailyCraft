<script setup lang="ts">
import { ref } from 'vue';

const version = ref('0.1.0');
const description = ref('AI可视化日志分析软件，帮助您追踪每日活动并生成智能日记。');

function openLink(url: string) {
  window.open(url, '_blank');
}
</script>

<template>
  <div class="about">
    <div class="about-content">
      <div class="about-header">
        <img src="/icon.png" alt="DailyCraft" class="about-icon" />
        <h1 class="about-title">DailyCraft</h1>
        <p class="about-version">版本 {{ version }}</p>
        <p class="about-desc">{{ description }}</p>
      </div>

      <div class="about-buttons">
        <button class="btn btn-primary">
          <span>⬇</span> 检查更新
        </button>
        <button class="btn btn-secondary" @click="openLink('https://github.com')">
          <span>📦</span> GitHub
        </button>
        <button class="btn btn-secondary">
          <span>👥</span> QQ群
        </button>
        <button class="btn btn-secondary">
          <span>📺</span> Bilibili
        </button>
      </div>

      <div class="about-links">
        <div class="link-item" @click="openLink('https://github.com')">
          <span class="link-icon">⭐</span>
          <span class="link-text">给个Star</span>
        </div>
        <div class="link-item">
          <span class="link-icon">💬</span>
          <span class="link-text">反馈问题</span>
        </div>
        <div class="link-item">
          <span class="link-icon">📢</span>
          <span class="link-text">推荐分享</span>
        </div>
        <div class="link-item">
          <span class="link-icon">❤️</span>
          <span class="link-text">赞赏支持</span>
        </div>
      </div>

      <div class="about-footer">
        <p class="copyright">© 2025 DailyCraft. All rights reserved.</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.about {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f5f5;
  overflow: auto;
}

.about-content {
  max-width: 600px;
  margin: 0 auto;
  padding: 40px 20px;
  text-align: center;
}

.about-header {
  margin-bottom: 32px;
}

.about-icon {
  width: 80px;
  height: 80px;
  object-fit: contain;
  margin-bottom: 16px;
}

.about-title {
  font-family: 'FZG', sans-serif;
  font-size: 28px;
  font-weight: 700;
  color: #1f2937;
  margin: 0 0 8px;
}

.about-version {
  font-size: 14px;
  color: #6b7280;
  margin: 0 0 16px;
}

.about-desc {
  font-size: 14px;
  color: #4b5563;
  margin: 0;
  line-height: 1.6;
}

.about-buttons {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin-bottom: 32px;
  flex-wrap: wrap;
}

.btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid #e5e7eb;
}

.btn-primary {
  background: #3b82f6;
  color: white;
  border-color: #3b82f6;
}

.btn-primary:hover {
  background: #2563eb;
}

.btn-secondary {
  background: white;
  color: #374151;
}

.btn-secondary:hover {
  background: #f3f4f6;
}

.about-links {
  background: white;
  border-radius: 12px;
  border: 1px solid #e5e7eb;
  overflow: hidden;
  margin-bottom: 32px;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  cursor: pointer;
  transition: background 0.2s;
  border-bottom: 1px solid #e5e7eb;
}

.link-item:last-child {
  border-bottom: none;
}

.link-item:hover {
  background: #f9fafb;
}

.link-icon {
  font-size: 18px;
}

.link-text {
  font-size: 14px;
  color: #374151;
}

.about-footer {
  margin-top: 20px;
}

.copyright {
  font-size: 12px;
  color: #9ca3af;
  margin: 0;
}
</style>
