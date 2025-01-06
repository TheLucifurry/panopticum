<script setup lang="ts">
import { Video } from '@/entities/common'
import { BezelProvider } from '@/shared/components/custom'
import { usePlayer } from '@/shared/modules'
import { useInteraction } from '@/shared/modules/interaction'
import { useEventListener, watchIgnorable } from '@vueuse/core'
import { computed, shallowRef, watch } from 'vue'

const videoPath = computed(() => convertFileSrc('______'))

const player = usePlayer()
const i10 = useInteraction()
const bezelProvider = shallowRef<InstanceType<typeof BezelProvider> | null>(null)
const videoRef = shallowRef<InstanceType<typeof Video> | null>(null)
const videoEl = computed(() => videoRef.value?.videoElement || null)

// Income bind
const refreshVolume = (v = player.volume) => videoEl.value && (videoEl.value.volume = v)
const refreshIsPlaying = (v = player.isPlaying) => v ? videoEl.value?.play() : videoEl.value?.pause()
const refreshRate = (v = player.rate) => videoEl.value && (videoEl.value.playbackRate = v)
const refreshCurrentTime = (v = player.currentTime) => videoEl.value && (videoEl.value.currentTime = v)
const refreshMute = (v = player.isMuted) => videoEl.value && (videoEl.value.muted = v)
watch(videoPath, v => videoEl.value && (videoEl.value.src = v))
watch(() => player.volume, refreshVolume)
watch(() => player.isPlaying, refreshIsPlaying)
watch(() => player.rate, refreshRate)
watch(() => player.isMuted, refreshMute)
const ignoringWatchExternal = watchIgnorable(() => player.currentTime, refreshCurrentTime).ignoreUpdates
useEventListener(videoEl, 'canplay', () => {
  if (videoEl.value) {
    refreshVolume()
    refreshIsPlaying()
    refreshRate()
    refreshMute()
    refreshCurrentTime()
  }
}, { once: true })

// Outcome bind
function onLoadedMetaData() {
  player.trackLengthTime = videoEl.value?.duration ?? player.trackLengthTime
}
function onTimeupdate() {
  ignoringWatchExternal(() => {
    player.currentTime = videoEl.value?.currentTime ?? player.currentTime
  })
}

i10.bezel.connect(bezelProvider)
</script>

<template>
  <div class="media-player">
    <Video
      ref="videoRef"
      src=""
      @loadedmetadata="onLoadedMetaData"
      @timeupdate="onTimeupdate"
    />
    <BezelProvider ref="bezelProvider" />
  </div>
</template>

<style lang="scss">
.media-player {
  width: 100%;
  height: 100%;
  position: relative;

  & > * {
    position: absolute;
  }
}
</style>
