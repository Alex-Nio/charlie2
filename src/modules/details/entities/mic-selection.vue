<script setup>
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';

  const selectedMicrophone = ref(0);
  const availableMicrophones = ref([]);
  const isDropdownOpen = ref(false);

  const setupMicrophone = async () => {
    try {
      availableMicrophones.value = await invoke('pv_get_audio_devices');
      selectedMicrophone.value = +Number(
        await invoke('db_read', { key: 'selected_microphone' })
      );
    } catch (err) {
      console.error(err);
    }
  };

  const fetchAvailableMicrophones = async () => {
    try {
      availableMicrophones.value = await invoke('pv_get_audio_devices');
    } catch (err) {
      console.error(err);
    }
  };

  const toggleDropdown = () => {
    isDropdownOpen.value = !isDropdownOpen.value;
  };

  const selectMicrophone = (index) => {
    selectedMicrophone.value = index;
    handleMicrophoneChange();
    isDropdownOpen.value = false;
  };

  const handleMicrophoneChange = async () => {
    try {
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

  onMounted(async () => {
    setInterval(fetchAvailableMicrophones, 1500);
    await setupMicrophone();
  });
</script>
<template>
  <div class="online">
    <div class="info">
      <span class="num">Микрофон:</span>
      <div
        class="custom-select"
        @click="toggleDropdown"
      >
        <div class="selected-option">
          {{
            availableMicrophones[selectedMicrophone]?.name ||
            'Выберите микрофон'
          }}
        </div>
        <div
          v-if="isDropdownOpen"
          class="dropdown"
        >
          <div
            v-for="(mic, i) in availableMicrophones"
            :key="i"
            @click="selectMicrophone(i)"
          >
            {{ mic.name }}
          </div>
        </div>
      </div>
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

  .custom-select {
    position: relative;
    cursor: pointer;
    user-select: none;
  }

  .selected-option {
    padding: 8px;
    border: 1px solid #ccc;
  }

  .dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    background-color: #fff;
    border: 1px solid #ccc;
    display: flex;
    flex-direction: column;
    max-height: 200px;
    overflow-y: auto;
    z-index: 1;

    div {
      padding: 8px;
      cursor: pointer;

      &:hover {
        background-color: #f0f0f0;
      }
    }
  }
</style>
