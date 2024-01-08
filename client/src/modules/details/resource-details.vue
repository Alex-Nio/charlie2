<script setup>
  // imports
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  // consts
  const resourcesRamUsage = ref('-');
  const selectedMicrophone = ref(0);
  const microphoneLabel = ref('');

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

  const setupMicrophone = async () => {
    const capitalizeFirstLetter = (string) => {
      return string.charAt(0).toUpperCase() + string.slice(1);
    };

    try {
      selectedMicrophone.value = +Number(
        await invoke('db_read', { key: 'selected_microphone' })
      );

      microphoneLabel.value = await invoke('pv_get_audio_device_name', {
        idx: selectedMicrophone.value,
      });

      nnDetails.value.wwEngine = capitalizeFirstLetter(
        await invoke('db_read', { key: 'selected_wake_word_engine' })
      );
    } catch (err) {
      console.error(err);
    }
  };

  // Lifecycle hook
  onMounted(async () => {
    setInterval(updateResourcesRamUsage, 1000);
    await setupMicrophone();
  });
</script>

<template>
  <div class="details-block">
    <!-- Микрофон -->
    <div class="online">
      <div class="info">
        <span class="num">Микрофон:</span>
        <small title="{{ microphoneLabel }}">{{ microphoneLabel }}</small>
      </div>
    </div>

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

  .info {
    display: flex;
    color: rgb(255, 255, 255);
    gap: 8px;
    z-index: 10;
  }

  .num {
    font-weight: bold;
    color: #00bf08;
  }
</style>
