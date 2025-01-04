import { defineComposable } from '@webshrine/vue'
import { shallowRef } from 'vue'
import { clamp } from 'webshrine'

export const DEFAULT_VOLUME = 0.5
export const DEFAULT_VOLUME_INC_STEP = 0.05
export const DEFAULT_PLAYBACK_SPEED = 1
export const DEFAULT_PLAYBACK_SPEED_STEP = 0.25
export const DEFAULT_PLAYBACK_SPEED_OPTIONS = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2]
export const DEFAULT_PLAYBACK_SPEED_RANGE = [0.1, 3] as const
export const DEFAULT_AUTOPLAY = false

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
      volumeInc(step = 1) {
        volume.value = clamp(volume.value + step * DEFAULT_VOLUME_INC_STEP, 0, 1)
      },
      playbackSpeed,
      playbackSpeedInc(step = 1) {
        playbackSpeed.value = clamp(playbackSpeed.value + step * DEFAULT_PLAYBACK_SPEED_STEP, ...DEFAULT_PLAYBACK_SPEED_RANGE)
      },
      isPlaying,
      isAutoplay,
      currentTime: 0,
      trackLengthTime: 500,
    }
  },
})
