<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import Phaser from 'phaser';
import PetScene from './scenes/PetScene';
import defaultPetConfig from './config/defaultPet';

const phaserContainer = ref<HTMLDivElement | null>(null);
let game: Phaser.Game | null = null;

const screenWidth = ref(window.screen.width);
const screenHeight = ref(window.screen.height);

function createGame() {
  if (!phaserContainer.value) return;

  const phaserConfig: Phaser.Types.Core.GameConfig = {
    type: Phaser.AUTO,
    parent: phaserContainer.value,
    backgroundColor: '#ffffff0',
    transparent: true,
    roundPixels: true,
    antialias: true,
    scale: {
      mode: Phaser.Scale.ScaleModes.RESIZE,
      width: screenWidth.value,
      height: screenHeight.value,
    },
    physics: {
      default: 'arcade',
      arcade: {
        debug: false,
        gravity: { x: 0, y: 200 },
      },
    },
    fps: {
      target: 30,
      min: 30,
      smoothStep: true,
    },
    scene: [PetScene],
    audio: {
      noAudio: true,
    },
    callbacks: {
      preBoot: (game) => {
        // 传入固定的单个宠物配置
        game.registry.set('spriteConfig', [defaultPetConfig]);
      },
    },
  };

  game = new Phaser.Game(phaserConfig);
}

function handleResize() {
  screenWidth.value = window.screen.width;
  screenHeight.value = window.screen.height;
}

onMounted(() => {
  window.addEventListener('resize', handleResize);
  createGame();
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  if (game) {
    game.destroy(true);
    game = null;
  }
  if (phaserContainer.value) {
    phaserContainer.value.innerHTML = '';
  }
});
</script>

<template>
  <div ref="phaserContainer" class="pet-overlay"></div>
</template>

<style scoped>
.pet-overlay {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  margin: 0;
  padding: 0;
  background: transparent;
}
</style>
