<script setup>
  import { ref } from "vue";
  import axios from "axios";
  import Reactor from "./modules/reactor/mod.vue";

  const apiResponse = ref("");

  const methods = {
    async callPythonScript() {
      console.log("Клик по кнопке!");
      try {
        const response = await axios.get("/run-python-script");
        apiResponse.value = response.data;
      } catch (error) {
        console.error("Ошибка при вызове Python-скрипта:", error);
      }
    },
  };

  // Устанавливаем серверное соединение для обновлений
  const eventSource = new EventSource("/recognition-updates");
  eventSource.onmessage = (event) => {
    apiResponse.value = event.data;
  };
</script>

<template>
  <div>
    <button @click="methods.callPythonScript">Вызвать Python Script</button>
    <p class="logger">{{ apiResponse }}</p>
  </div>
  <reactor />
</template>

<style lang="scss">
  .logger {
    font-size: 22px;
    text-align: center;
    color: #ffffff !important;
  }
</style>
