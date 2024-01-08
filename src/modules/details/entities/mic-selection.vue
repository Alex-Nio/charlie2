<script setup>
  // imports
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  // consts
  const selectedMicrophone = ref(0);
  const microphoneLabel = ref('');
  const availableMicrophones = ref([]);

  const setupMicrophone = async () => {
    try {
      // Fetch available microphones
      availableMicrophones.value = await invoke('pv_get_audio_devices');

      // Set selected microphone
      selectedMicrophone.value = +Number(
        await invoke('db_read', { key: 'selected_microphone' })
      );

      // Set microphone label
      microphoneLabel.value = await invoke('pv_get_audio_device_name', {
        idx: selectedMicrophone.value,
      });
    } catch (err) {
      console.error(err);
    }
  };

  // Function to fetch available microphones
  const fetchAvailableMicrophones = async () => {
    try {
      availableMicrophones.value = await invoke('pv_get_audio_devices');
    } catch (err) {
      console.error(err);
    }
  };

  // Function to handle microphone change
  const handleMicrophoneChange = async () => {
    try {
      console.log('change microphone', selectedMicrophone.value);

      // Update the selected microphone in the database
      await invoke('db_write', {
        key: 'selected_microphone',
        val: selectedMicrophone.value.toString(),
      });

      await invoke('update_selected_microphone', {
        index: selectedMicrophone.value,
      });
    } catch (err) {
      console.error(err);
    }
  };

  // Lifecycle hook
  onMounted(async () => {
    setInterval(fetchAvailableMicrophones, 1500);
    await setupMicrophone();
  });
</script>

<template>
  <!-- Микрофон -->
  <div class="online">
    <div class="info">
      <span class="num">Микрофон:</span>
      <select
        v-model="selectedMicrophone"
        @change="handleMicrophoneChange"
      >
        <option
          v-for="(mic, i) in availableMicrophones"
          :key="i"
          :value="i"
        >
          {{ mic.name }}
        </option>
      </select>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .info {
    display: flex;
    align-items: center;
    color: rgb(255, 255, 255);
    gap: 8px;
    z-index: 10;
  }
</style>
