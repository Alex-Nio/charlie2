<script setup>
  import Reactor from "./modules/reactor/mod.vue";
  import { ref, onMounted } from "vue";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  const isListening = ref(true);
  const isReactorActive = ref(false);
  const resourcesRamUsage = ref("-");
  const assistantVoiceVal = "jarvis-og";
  const selectedMicrophone = ref(0);
  const microphoneLabel = ref("");

  // State
  const nnDetails = ref({
    sttEngine: "Vosk",
    ttsEngine: "Silero",
    gptEngine: "gpt-3.5-turbo-1106",
  });

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

  const updateResourcesRamUsage = async () => {
    try {
      resourcesRamUsage.value = Number(await invoke("get_current_ram_usage")).toFixed(2);
    } catch (err) {
      console.error(err);
    }
  };

  const setupMicrophone = async () => {
    const capitalizeFirstLetter = (string) => {
    return string.charAt(0).toUpperCase() + string.slice(1);
  };

    try {
      selectedMicrophone.value = +Number(await invoke("db_read", { key: "selected_microphone" }));
      microphoneLabel.value = await invoke("pv_get_audio_device_name", {
        idx: selectedMicrophone.value,
      });

      nnDetails.value.wwEngine = capitalizeFirstLetter(await invoke("db_read", { key: "selected_wake_word_engine" }));
    } catch (err) {
      console.error(err);
    }
  };

  // Lifecycle hook
  onMounted(async () => {
    setInterval(updateResourcesRamUsage, 1000);
    await setupMicrophone();
  });

  // Initial setup
  startListening();
  listenToAudioPlay();
</script>

<template>
  <Reactor :isReactorActive="isReactorActive" />
  <div>
    <!-- Микрофон -->
    <div class="online">
      <div class="pulse">
        <div class="wave"></div>
      </div>
      <div class="info">
        <span class="num">Микрофон:</span>
        <small title="{{ microphoneLabel }}">{{ microphoneLabel }}</small>
      </div>
    </div>

    <!-- Нейросети -->
    <div class="files">
      <div class="pulse">
        <div class="wave"></div>
      </div>
      <div class="info">
        <span class="num">Нейросети:</span>
        <small>{{ nnDetails.sttEngine }} + {{ nnDetails.ttsEngine }} + {{ nnDetails.gptEngine }}</small>
      </div>
    </div>

    <!-- Ресурсы -->
    <div class="info">
      <span class="num">Ресурсы:</span>
      <small> RAM {{ resourcesRamUsage }}mb </small>
    </div>
  </div>
</template>

<style lang="scss">
body {
  font-family: 'Roboto', sans-serif;
  overflow-x: hidden;
  backface-visibility: hidden;
  -webkit-backface-visibility: hidden;
}

.logger {
  font-size: 22px;
  text-align: center;
  color: #ffffff !important;
}

.info {
  display: flex;
  color: rgb(255, 255, 255);
  gap: 8px;
  z-index: 10;

  & small {
    font-size: 12px;
  }
}

.num {
  font-size: 12px;
  font-weight: bold;
  color: #00bf08;
}
</style>
