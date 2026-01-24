<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  visible: boolean;
  message: string;
  type?: 'success' | 'error' | 'info' | 'warning';
  duration?: number;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const show = ref(props.visible);

watch(() => props.visible, (val) => {
  show.value = val;
  if (val && props.duration !== 0) {
    setTimeout(() => {
      emit('close');
    }, props.duration || 3000);
  }
});

const iconMap = {
  success: '✓',
  error: '✕',
  info: 'ℹ',
  warning: '⚠'
};

const colorMap = {
  success: '#10b981',
  error: '#ef4444',
  info: '#3b82f6',
  warning: '#f59e0b'
};
</script>

<template>
  <Teleport to="body">
    <Transition name="toast">
      <div v-if="show" class="toast-overlay">
        <div class="toast-container" :style="{ '--toast-color': colorMap[type || 'info'] }">
          <div class="toast-icon">{{ iconMap[type || 'info'] }}</div>
          <div class="toast-message">{{ message }}</div>
          <button class="toast-close" @click="$emit('close')">×</button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.toast-overlay {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10000;
}

.toast-container {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 20px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
  border: 1px solid #e5e7eb;
  min-width: 280px;
}

.toast-icon {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: var(--toast-color);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: bold;
  flex-shrink: 0;
}

.toast-message {
  flex: 1;
  font-size: 14px;
  color: #374151;
  line-height: 1.5;
}

.toast-close {
  background: none;
  border: none;
  font-size: 20px;
  color: #9ca3af;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.toast-close:hover {
  color: #6b7280;
}

.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(-20px);
}
</style>
