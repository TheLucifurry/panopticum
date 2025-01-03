import { defineComposable } from '@webshrine/vue'
import { shallowRef } from 'vue'

const DEFAULT_VOLUME = 0.5
const DEFAULT_PLAYBACK_SPEED = 1
const DEFAULT_AUTOPLAY = false

export const usePlayer = defineComposable({
  singleton: true,
  flat: true,
  setup() {
    const volume = shallowRef(DEFAULT_VOLUME)
    const playbackSpeed = shallowRef(DEFAULT_PLAYBACK_SPEED)
    const isPlaying = shallowRef(false)
    const isAutoplay = shallowRef(DEFAULT_AUTOPLAY)

    return {
      volume,
      playbackSpeed,
      isPlaying,
      isAutoplay,
      currentTime: 0,
      trackLengthTime: 500,
    }
  },
})
