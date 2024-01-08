<script setup>
  // imports
  import Mic from './entities/mic-selection.vue';
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  // consts
  const resourcesRamUsage = ref('-');

  // State
  const nnDetails = ref({
    sttEngine: 'Vosk',
    ttsEngine: 'RVC Charlie TTS',
    gptEngine: 'gpt-3.5-turbo-1106',
  });

  const updateResourcesRamUsage = async () => {
    try {
      resourcesRamUsage.value = Number(
        await invoke('get_current_ram_usage')
      ).toFixed(2);
    } catch (err) {
      console.error(err);
    }
  };

  // Lifecycle hook
  onMounted(async () => {
    setInterval(updateResourcesRamUsage, 1000);
  });
</script>

<template>
  <div class="details-block">
    <!-- Микрофон -->
    <Mic />

    <!-- Нейросети -->
    <div class="files">
      <div class="info">
        <span class="num">Нейросети:</span>
        <small>
          {{ nnDetails.sttEngine }} + {{ nnDetails.ttsEngine }} +
          {{ nnDetails.gptEngine }}
        </small>
      </div>
    </div>

    <!-- Ресурсы -->
    <div class="info">
      <span class="num">Ресурсы:</span>
      <small> RAM {{ resourcesRamUsage }}mb </small>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .details-block {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 8px;
    font-size: 16px;
    padding: 12px;
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    background-color: rgba(17, 25, 40, 0.75);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.125);
  }

  .num {
    font-weight: bold;
    color: #00bf08;
  }

  select {
    padding: 4px;
    font-size: 14px;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #fff;
    color: #333;
    cursor: pointer;
    transition: border-color 0.3s;

    &:focus {
      outline: none;
      border-color: #00bf08;
    }

    &:hover {
      border-color: #00bf08;
    }
  }
</style>
