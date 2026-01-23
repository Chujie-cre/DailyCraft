<script setup lang="ts">
import { ref } from 'vue';

const version = ref('0.1.0');
const description = ref('AI可视化日志分析软件，帮助您追踪每日活动并生成智能日记。');

const contactForm = ref({
  email: '',
  message: ''
});
const isSubmitting = ref(false);
const submitSuccess = ref(false);

function openLink(url: string) {
  window.open(url, '_blank');
}

async function submitContact() {
  if (!contactForm.value.email || !contactForm.value.message) {
    alert('请填写邮箱和消息内容');
    return;
  }
  
  isSubmitting.value = true;
  try {
    const response = await fetch('https://formsubmit.co/ajax/2425739349@qq.com', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      },
      body: JSON.stringify({
        email: contactForm.value.email,
        message: contactForm.value.message
      })
    });
    
    if (response.ok) {
      submitSuccess.value = true;
      contactForm.value = { email: '', message: '' };
      setTimeout(() => { submitSuccess.value = false; }, 3000);
    }
  } catch (e) {
    console.error('提交失败:', e);
    alert('提交失败，请稍后重试');
  } finally {
    isSubmitting.value = false;
  }
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
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
          检查更新
        </button>
        <button class="btn btn-secondary" @click="openLink('https://github.com')">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
          GitHub
        </button>
        <button class="btn btn-secondary">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M23 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path></svg>
          QQ群
        </button>
        <button class="btn btn-secondary">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="7" width="20" height="15" rx="2" ry="2"></rect><polyline points="17 2 12 7 7 2"></polyline></svg>
          Bilibili
        </button>
      </div>

      <div class="two-column-layout">
        <div class="about-links">
          <div class="link-item" @click="openLink('https://github.com')">
            <span class="link-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="#fbbf24" stroke="#fbbf24" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg></span>
            <span class="link-text">给个Star</span>
          </div>
          <div class="link-item">
            <span class="link-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#3b82f6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg></span>
            <span class="link-text">反馈问题</span>
          </div>
          <div class="link-item">
            <span class="link-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#f97316" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 2L11 13"></path><polygon points="22 2 15 22 11 13 2 9 22 2"></polygon></svg></span>
            <span class="link-text">贡献项目</span>
          </div>
          <div class="link-item">
            <span class="link-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="#ef4444" stroke="#ef4444" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path></svg></span>
            <span class="link-text">赞赏支持</span>
          </div>
        </div>

        <div class="contact-section">
        <div class="contact-form">
          <div class="shadow__input"></div>
          <div class="form-row">
            <button class="input__button__shadow avatar-btn" type="button">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#000000" width="20px" height="20px">
                <path d="M0 0h24v24H0z" fill="none"></path>
                <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path>
              </svg>
            </button>
            <div class="input__container flex-1" data-label="EMAIL">
              <input
                v-model="contactForm.email"
                type="email"
                class="input__search"
                placeholder="请输入邮箱"
              />
            </div>
          </div>
          <div class="form-group">
            <div class="input__container input__container--message" data-label="MESSAGE">
              <textarea
                v-model="contactForm.message"
                class="input__search input__textarea"
                placeholder="请输入您的留言"
                rows="4"
              ></textarea>
            </div>
          </div>
          <button 
            class="input__button__shadow" 
            @click="submitContact"
            :disabled="isSubmitting"
          >
            <svg v-if="!isSubmitting" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="#000000" width="20px" height="20px">
              <path d="M0 0h24v24H0z" fill="none"></path>
              <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
            </svg>
            <span v-if="isSubmitting">发送中...</span>
            <span v-else>发送</span>
          </button>
          <p v-if="submitSuccess" class="success-msg">发送成功！</p>
        </div>
        </div>
      </div>

      <div class="about-footer">
        <p class="copyright">© 2026 DailyCraft 💜 ChuJie All rights reserved.</p>
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

.two-column-layout {
  display: flex;
  gap: 24px;
  margin-bottom: 32px;
  align-items: flex-start;
}

.about-links {
  position: relative;
  background: #f0f0f0;
  padding: 15px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  border: 4px solid #000;
  flex: 1;
  min-width: 200px;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
  transform-style: preserve-3d;
  transform: rotateX(10deg) rotateY(10deg);
  perspective: 1000px;
  box-shadow: -10px 10px 0 #000;
}

.about-links:hover {
  transform: rotateX(5deg) rotateY(-1deg) scale(1.02);
  box-shadow: -25px 25px 0 -5px #e9b50b, -25px 25px 0 0 #000;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 12px;
  min-height: 57px;
  padding: 12px 15px;
  cursor: pointer;
  background: #fff;
  border: 3px solid #000;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
  transform: translateZ(10px);
}

.link-item:hover {
  background: #f0f0f0;
  transform: translateZ(20px) translateX(3px) translateY(-3px);
  box-shadow: -3px 3px 0 0 #000;
}

.link-icon {
  font-size: 18px;
}

.link-text {
  font-size: 14px;
  color: #000;
  font-weight: 600;
}

.about-footer {
  margin-top: 20px;
}

.copyright {
  font-size: 12px;
  color: #9ca3af;
  margin: 0;
}

.contact-section {
  flex: 1;
  min-width: 280px;
}

.contact-form {
  position: relative;
  background: #f0f0f0;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
  border: 4px solid #000;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
  transform-style: preserve-3d;
  transform: rotateX(10deg) rotateY(-10deg);
  perspective: 1000px;
  box-shadow: 10px 10px 0 #000;
}

.contact-form:hover {
  transform: rotateX(5deg) rotateY(1deg) scale(1.02);
  box-shadow: 25px 25px 0 -5px #e9b50b, 25px 25px 0 0 #000;
}

.shadow__input {
  content: "";
  position: absolute;
  width: 100%;
  height: 100%;
  left: 0;
  bottom: 0;
  z-index: -1;
  transform: translateZ(-50px);
  background: linear-gradient(45deg, rgba(255, 107, 107, 0.4) 0%, rgba(255, 107, 107, 0.1) 100%);
  filter: blur(20px);
}

.form-group {
  width: 100%;
}

.form-row {
  display: flex;
  align-items: flex-start;
  gap: 15px;
}

.flex-1 {
  flex: 1;
}

.avatar-btn {
  padding: 15px;
  flex-shrink: 0;
}

.input__container {
  position: relative;
  width: 100%;
}

.input__container::before {
  content: attr(data-label);
  position: absolute;
  top: -12px;
  left: 10px;
  background: #e9b50b;
  color: #000;
  font-weight: bold;
  padding: 2px 8px;
  font-size: 12px;
  transform: translateZ(50px);
  z-index: 4;
  border: 2px solid #000;
}

.input__search {
  width: 100%;
  outline: none;
  border: 3px solid #000;
  padding: 15px;
  font-size: 16px;
  background: #fff;
  color: #000;
  transform: translateZ(10px);
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
  position: relative;
  z-index: 3;
  font-family: "Roboto", Arial, sans-serif;
}

.input__search::placeholder {
  color: #666;
  font-weight: bold;
}

.input__search:hover,
.input__search:focus {
  background: #f0f0f0;
  transform: translateZ(20px) translateX(-3px) translateY(-3px);
  box-shadow: 3px 3px 0 0 #000;
}

.input__textarea {
  resize: vertical;
  min-height: 100px;
}

.input__button__shadow {
  cursor: pointer;
  border: 3px solid #000;
  background: #e9b50b;
  transition: all 400ms cubic-bezier(0.23, 1, 0.32, 1);
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  transform: translateZ(20px);
  position: relative;
  z-index: 3;
  font-weight: bold;
  font-size: 14px;
  align-self: flex-end;
}

.input__button__shadow:hover {
  background: #ffc107;
  transform: translateZ(10px) translateX(-5px) translateY(-5px);
  box-shadow: 5px 5px 0 0 #000;
}

.input__button__shadow:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input__button__shadow svg {
  fill: #000;
  width: 20px;
  height: 20px;
}

.success-msg {
  color: #22c55e;
  font-weight: 600;
  margin: 0;
  text-align: center;
}
</style>
