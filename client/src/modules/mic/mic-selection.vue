<script setup>
  // imports
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  // Consts
  const availableMicrophones = ref([]);
  const selectedMicrophone = ref(0);

  //! TESTS
  const handleMicrophoneChange = async () => {
    try {
      getMicrophoneList();

      const selectedIndex = availableMicrophones.value.indexOf(
        selectedMicrophone.value
      );

      console.log(selectedIndex);

      await invoke('update_selected_microphone', { index: selectedIndex });
    } catch (error) {
      console.error('Error updating selected microphone:', error);
    }
  };

  const getMicrophoneList = async () => {
    try {
      const devices = await invoke('pv_get_audio_devices');
      availableMicrophones.value = devices;
    } catch (error) {
      console.error('Error getting microphone list:', error);
    }
  };

  // Call the function to get the microphone list when needed (e.g., component mounted)
  onMounted(() => {
    getMicrophoneList();
  });
</script>

<template>
  <div>
    <label for="microphone">Select Microphone:</label>
    <select
      v-model="selectedMicrophone"
      @change="handleMicrophoneChange"
    >
      <option
        v-for="(mic, i) in availableMicrophones"
        :key="i"
        :value="mic"
      >
        {{ mic }}
      </option>
    </select>
  </div>
</template>

<style lang="scss" scoped></style>
