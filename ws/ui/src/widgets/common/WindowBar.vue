<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow()
</script>

<template>
  <div class="window-bar">
    <div>
      <slot></slot>
    </div>
    <div data-tauri-drag-region></div>
    <div>
      <div class="window-bar__button" @click="appWindow.minimize()">
        <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
      </div>
      <div class="window-bar__button" @click="appWindow.toggleMaximize()">
        <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
      </div>
      <div class="window-bar__button" @click="appWindow.close()">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </div>
    </div>
  </div>
</template>

<style lang="scss">
$height: 40px;

.window-bar {
  height: $height;
  width: 100%;
  user-select: none;
  display: grid;
  top: 0;
  left: 0;
  right: 0;
  grid-template-columns: max-content 1fr max-content;

  &__button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: $height;
    height: $height;
    user-select: none;
    -webkit-user-select: none;
  }

  &__button:hover {
    background: #ffffff1c;
  }

  & > div {
    display: flex;
    align-items: center;
  }
}
</style>
