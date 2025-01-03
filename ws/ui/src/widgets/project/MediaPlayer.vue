<script setup lang="ts">
import { Video } from '@/entities/common'
import { usePlayer } from '@/shared/modules'
import { convertFileSrc } from '@tauri-apps/api/core'
import { syncRefs, useEventListener, useMediaControls } from '@vueuse/core'
import { computed, shallowRef, watch } from 'vue'

const videoPath = computed(() => convertFileSrc('______'))

const player = usePlayer()
const videoRef = shallowRef<InstanceType<typeof Video> | null>(null)
const videoEl = computed(() => videoRef.value?.videoElement || null)

const mediaControls = useMediaControls(videoEl, { src: videoPath })

syncRefs(() => player.volume, mediaControls.volume)
syncRefs(() => player.isPlaying, mediaControls.playing)
watch(mediaControls.currentTime, v => player.currentTime = v)

useEventListener(videoEl, 'loadedmetadata', () => {
  player.trackLengthTime = videoEl.value?.duration ?? player.trackLengthTime
})
</script>

<template>
  <div class="media-player">
    <Video ref="videoRef" />
  </div>
</template>

<style lang="scss">
.media-player {
  width: 100%;
  height: 100%;
}
</style>
