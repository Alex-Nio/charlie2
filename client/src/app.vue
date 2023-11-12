<script setup>
  import axios from "axios";
  import Reactor from "./modules/reactor/mod.vue";

  const data = () => ({
    apiResponse: "",
  });

  const methods = {
    async callPythonScript() {
      // debugger;
      console.log("Клик по кнопке!");
      try {
        const response = await axios.get("/run-python-script");
        this.apiResponse = response.data;
      } catch (error) {
        console.error("Ошибка при вызове Python-скрипта:", error);
      }
    },
  };
</script>

<template>
  <div>
    <button @click="methods.callPythonScript">Вызвать Python Script</button>
    <p>{{ apiResponse }}</p>
  </div>
  <reactor />
</template>

<style lang="scss"></style>
