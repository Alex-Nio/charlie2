<script setup>
  // imports
  import { ref } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  // Components
  import Titlebar from "./modules/titlebar/titlebar.vue";
  import Details from "./modules/details/details.vue";
  import Reactor from "./modules/reactor/reactor.vue";

  // Consts
  const isListening = ref(true);
  const isReactorActive = ref(false);
  const assistantVoiceVal = "jarvis-og";

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

    let filename = "sound/" + assistantVoiceVal + "/" + event.payload["data"] + ".wav";

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

  // Initial setup
  startListening();
  listenToAudioPlay();
</script>

<template>
  <Titlebar />

  <div class="full-wrapper">
    <Reactor :isReactorActive="isReactorActive" />
    <Details />
  </div>
</template>

<style lang="scss">
#app {
  margin-top: 48px;
}
</style>
