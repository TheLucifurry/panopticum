<script setup lang="ts">
import { shallowRef } from 'vue'

const {
  poster = '',
  isLoop = false,
} = defineProps<{
  poster?: string
  isLoop?: boolean
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
      class="video"
      :class="{
        'video--vertical': isVertical,
      }"
      crossorigin="anonymous"
      :poster="poster"
      :loop="isLoop"
      @loadedmetadata="onLoadedMetaData"
      @timeupdate="emit('timeupdate', $event)"
    />
  </div>
</template>

<style lang="scss">
.video {
  max-height: 100%;
  max-width: 100%;

  width: 100%;
  height: max-content;
  &--vertical {
    height: 100%;
    width: max-content;
  }

  &__wrapper {
    position: relative;
    width: 100%;
    height: 100%;
    background-color: #000;
    display: flex;
    justify-content: center;
    align-items: center;
  }
}
</style>
