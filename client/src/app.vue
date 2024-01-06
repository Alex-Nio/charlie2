<script setup>
  // imports
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  // Components
  import Titlebar from "./modules/titlebar/titlebar.vue";
  import Details from "./modules/details/details.vue";
  import Reactor from "./modules/reactor/reactor.vue";

  // Consts
  const isListening = ref(true);
  const isReactorActive = ref(false);
  const isTTSActive = ref(false);
  const assistantVoiceVal = "charlie-voices";

  // Functions
  const startListening = async () => {
    try {
      await invoke("start_listening");

      isListening.value = true;
      isReactorActive.value = false;
    } catch (error) {
      isListening.value = false;
      isReactorActive.value = false;

      console.error(error);
    }
  };

  const stopListening = async (callback) => {
    try {
      await invoke("stop_listening");
      isListening.value = false;
      isReactorActive.value = false;
      if (callback) {
        callback();
      }
    } catch (error) {
      console.error(error);
    }
  };

  const playAudio = async (event) => {
    isReactorActive.value = true;

    let folder = event.payload.folder;

    console.log(folder);

    let filename = "sound/" + assistantVoiceVal + "/" + folder + "/" + event.payload["data"] + ".wav";

    try {
      await invoke("play_sound", { filename, sleep: true });
    } catch (error) {
      console.error(error);
    }

    setTimeout(() => {
      isReactorActive.value = false;
    }, 500);
  };

  const listenToAudioPlay = async () => {
      await listen("audio-play", playAudio);
  };

  const listenToTtsStart = async () => {
    await listen("tts-started", handleTTSStarted);
  };

  const listenToTtsStop = async () => {
    await listen("tts-stopped", handleTTSStopped);
  };

  // Функция, которая будет вызываться при событии tts_started
  const handleTTSStarted = () => {
      // Обновляем состояние в соответствии с вашими нуждами
      isTTSActive.value = true;
      console.log('Обработка TTS началась...');
  };

  const handleTTSStopped = () => {
      // Обновляем состояние в соответствии с вашими нуждами
      isTTSActive.value = false;
      console.log('Обработка TTS закончилась...');
  };

  // Initial setup
  startListening();
  listenToAudioPlay();
  listenToTtsStart();
  listenToTtsStop();
</script>

<template>
  <Titlebar />

  <div class="full-wrapper">
    <Reactor :isReactorActive="isReactorActive" :isTTSActive="isTTSActive" />
    <Details />
  </div>
</template>

<style lang="scss">
#app {
  margin-top: 48px;
}
</style>
