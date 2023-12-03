<script setup>
  import { appWindow } from "@tauri-apps/api/window";
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar__inner">
      <div @click="appWindow.minimize()" class="titlebar-button" id="titlebar-minimize">
        <i class="icon-window-minimize"></i>
      </div>
      <div @click="appWindow.toggleMaximize()" class="titlebar-button" id="titlebar-maximize">
        <i class="icon-window-maximize"></i>
      </div>
      <div @click="appWindow.close()" class="titlebar-button" id="titlebar-close">
        <i class="icon-window-close"></i>
      </div>
    </div>
  </div>
</template>

<style lang='scss' scoped>
@import './../../assets/styles/config/imports.scss';

.titlebar {
  height: 48px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  background: url('./../../assets/images/titlebar-background.gif') no-repeat center / cover;

  &__inner {
    display: flex;
    justify-content: flex-end;
    gap: 14px;
    width: max-content;
    margin-right: 22px;
  }

  i {
    width: 32px;
    height: 32px;
    color: $white;
    @include defaultTransition;

    &::before {
      font-size: 32px;
    }
  }

  button {
    display: inline-flex;
    width: 32px;
    height: 32px;
  }
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;

  & img {
    color: #ffffff;
  }
}

.titlebar-button:hover {
  cursor: pointer;

  & i {
    color: rgb(0, 255, 229);
    @include defaultTransition;
  }
}
</style>
