import type { IMedia } from '@/shared/repositories'
import { registerPlayerKeybindings } from '@/features/player'
import { useIncrementable } from '@/shared/composables'
import { useToggle } from '@vueuse/core'
import { defineModule } from '@webshrine/vue'
import { shallowRef } from 'vue'

export const DEFAULT_VOLUME = 0.5
export const DEFAULT_VOLUME_INC_STEP = 0.05
export const DEFAULT_PLAYBACK_SPEED = 1
export const DEFAULT_PLAYBACK_SPEED_STEP = 0.25
export const DEFAULT_PLAYBACK_SPEED_OPTIONS = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2]
export const DEFAULT_PLAYBACK_SPEED_RANGE = [0.25, 3] as const
export const DEFAULT_AUTOPLAY = false

export const usePlayer = defineModule(() => {
  const currentMedia = shallowRef<IMedia | null>(null)

  const volume = shallowRef(DEFAULT_VOLUME)
  const volumeChange = useIncrementable(volume, { step: DEFAULT_VOLUME_INC_STEP, min: 0, max: 1 })

  const rate = shallowRef(DEFAULT_PLAYBACK_SPEED)
  const rateChange = useIncrementable(rate, {
    step: DEFAULT_PLAYBACK_SPEED_STEP,
    min: DEFAULT_PLAYBACK_SPEED_RANGE[0],
    max: DEFAULT_PLAYBACK_SPEED_RANGE[1],
  })

  const trackLengthTime = shallowRef(500)
  const currentTime = shallowRef(DEFAULT_PLAYBACK_SPEED)
  const currentTimeChange = useIncrementable(currentTime, {
    step: 5,
    min: 0,
    max: trackLengthTime.value,
  })

  const [isPlaying, togglePlaying] = useToggle()
  const [isAutoplay, toggleAutoplay] = useToggle(DEFAULT_AUTOPLAY)
  const [isMuted, toggleMuted] = useToggle()

  const currentTimeSetByPercent = (percent: number) => {
    currentTime.value = Math.round(trackLengthTime.value * percent)
  }

  const setCurrentMedia = (media: IMedia) => {
    currentTime.value = 0
    currentMedia.value = media
  }

  return {
    currentMedia,
    setCurrentMedia,
    volume,
    volumeChange,
    rate,
    rateChange,
    isPlaying,
    togglePlaying,
    isAutoplay,
    toggleAutoplay,
    isMuted,
    toggleMuted,
    currentTime,
    currentTimeChange,
    currentTimeSetByPercent,
    trackLengthTime,
  }
})

// TODO: implement plugin system for defineModule
registerPlayerKeybindings(usePlayer())
