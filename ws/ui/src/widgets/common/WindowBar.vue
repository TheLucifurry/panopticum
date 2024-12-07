<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow()
</script>

<template>
  <div data-tauri-drag-region class="window-bar">
    <div class="window-bar__button" @click="appWindow.minimize()">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div class="window-bar__button" @click="appWindow.toggleMaximize()">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div class="window-bar__button" @click="appWindow.close()">
      <img
        src="https://api.iconify.design/mdi:close.svg"
        alt="close"
      />
    </div>
  </div>
</template>

<style lang="scss">
.window-bar {
  height: 30px;
  background: #9e03ba;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;

  &__button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    user-select: none;
    -webkit-user-select: none;
  }

  &__button:hover {
    background: #ffffff1c;
  }
}
</style>
