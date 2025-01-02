<script setup lang="ts">
import { shallowRef } from 'vue';

const videoElement = shallowRef<HTMLVideoElement | null>(null)

const {
  poster = '',
  isLoop = false,
} = defineProps<{
  poster?: string;
  isLoop?: boolean;
}>()

defineExpose({ videoElement })
</script>

<template>
  <div class="video__wrapper">
    <video
      ref="videoElement"
      class="video"
      type="video/webm"
      crossorigin="anonymous"
      :poster="poster"
      :loop="isLoop"
    />
  </div>
</template>

<style lang="scss">
.video {
  max-height: 100%;
  max-width: 100%;

  &__wrapper {
    width: 100%;
    height: 100%;
    background-color: #000;
    display: flex;
    justify-content: center;
    align-items: center;
  }
}
</style>
