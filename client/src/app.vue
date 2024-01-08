<script setup>
  // imports
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';

  // Components
  import Titlebar from './modules/titlebar/title-bar.vue';
  import Details from './modules/details/resource-details.vue';
  import Reactor from './modules/reactor/reactor-decoration.vue';

  // Consts
  const isListening = ref(true);
  const isReactorActive = ref(false);
  const isTTSActive = ref(false);
  const availableMicrophones = ref([]);
  const selectedMicrophone = ref(0);
  const assistantVoiceVal = 'charlie-voices';

  // Functions
  const startListening = async () => {
    try {
      await invoke('start_listening');

      isListening.value = true;
      isReactorActive.value = false;
    } catch (error) {
      isListening.value = false;
      isReactorActive.value = false;

      // console.error(error);
    }
  };

  // const stopListening = async (callback) => {
  //   try {
  //     await invoke('stop_listening');
  //     isListening.value = false;
  //     isReactorActive.value = false;
  //     if (callback) {
  //       callback();
  //     }
  //   } catch (error) {
  //     console.error(error);
  //   }
  // };

  const playAudio = async (event) => {
    isReactorActive.value = true;

    let { folder } = event.payload;

    let filename =
      'sound/' +
      assistantVoiceVal +
      '/' +
      folder +
      '/' +
      event.payload['data'] +
      '.wav';

    try {
      await invoke('play_sound', { filename, sleep: true });
    } catch (error) {
      // console.error(error);
    }

    setTimeout(() => {
      isReactorActive.value = false;
    }, 500);
  };

  const listenToAudioPlay = async () => {
    await listen('audio-play', playAudio);
  };

  const listenToTtsStart = async () => {
    await listen('tts-started', handleTTSStarted);
  };

  const listenToTtsStop = async () => {
    await listen('tts-stopped', handleTTSStopped);
  };

  // Функция, которая будет вызываться при событии tts_started
  const handleTTSStarted = () => {
    // Обновляем состояние в соответствии с вашими нуждами
    isTTSActive.value = true;
  };

  const handleTTSStopped = () => {
    // Обновляем состояние в соответствии с вашими нуждами
    isTTSActive.value = false;
  };

  // Initial setup
  startListening();
  listenToAudioPlay();
  listenToTtsStart();
  listenToTtsStop();

  //! TESTS
  const handleMicrophoneChange = async () => {
    try {
      getMicrophoneList();

      const selectedIndex = availableMicrophones.value.indexOf(
        selectedMicrophone.value
      );

      // console.log(selectedIndex);

      await invoke('update_selected_microphone', { index: selectedIndex });
    } catch (error) {
      // console.error('Error updating selected microphone:', error);
    }
  };

  const getMicrophoneList = async () => {
    try {
      const devices = await invoke('pv_get_audio_devices');
      availableMicrophones.value = devices;
    } catch (error) {
      // console.error('Error getting microphone list:', error);
    }
  };

  // Call the function to get the microphone list when needed (e.g., component mounted)
  onMounted(() => {
    getMicrophoneList();
  });
</script>

<template>
  <Titlebar />

  <div class="full-wrapper">
    <Reactor
      :isReactorActive="isReactorActive"
      :isTTSActive="isTTSActive"
    />
    <Details />

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
  </div>
</template>

<style lang="scss">
  #app {
    margin-top: 48px;
  }
</style>
