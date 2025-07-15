<script setup lang="ts">
import { shallowRef } from 'vue'

const {
  poster = '',
  isLoop = false,
  src,
} = defineProps<{
  poster?: string
  isLoop?: boolean
  src: string
}>()

const emit = defineEmits<{
  (e: 'loadedmetadata', payload: Event): void
  (e: 'timeupdate', payload: Event): void
}>()

const videoElement = shallowRef<HTMLVideoElement | null>(null)
const isVertical = shallowRef(false)

function onLoadedMetaData(payload: Event) {
  const video = videoElement.value
  isVertical.value = video ? video.videoWidth < video.videoHeight : false
  emit('loadedmetadata', payload)
}

defineExpose({ videoElement })
</script>

<template>
  <div class="video__wrapper">
    <video
      ref="videoElement"
      :src="src"
      class="video"
      :class="{
        vertical: isVertical,
      }"
      crossorigin="anonymous"
      :poster="poster"
      :loop="isLoop"
      @loadedmetadata="onLoadedMetaData"
      @timeupdate="emit('timeupdate', $event)"
    />
  </div>
</template>

<style>
.video__wrapper {
  position: relative;
  width: 100%;
  height: 100%;
  background-color: #000;
  display: flex;
  justify-content: center;
  align-items: center;
}

.video {
  max-height: 100%;
  max-width: 100%;

  width: 100%;
  height: max-content;

  .vertical {
    height: 100%;
    width: max-content;
  }
}
</style>
