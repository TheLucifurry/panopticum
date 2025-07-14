import type { IContentMedia } from '@panopticum/schemas'
import { useToggle } from '@vueuse/core'
import { defineModule } from '@webshrine/vue'
import { shallowRef } from 'vue'
import { timesMap } from 'webshrine'
import { useKeyboard } from '@/modules/keyboard'
import { useIncrementable } from '@/shared/composables'

export const DEFAULT_DURATION = 4 * 60 + 4
export const DEFAULT_VOLUME = 0.5
export const DEFAULT_VOLUME_INC_STEP = 0.05
export const DEFAULT_PLAYBACK_SPEED = 1
export const DEFAULT_PLAYBACK_SPEED_STEP = 0.25
export const DEFAULT_PLAYBACK_SPEED_OPTIONS = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2]
export const DEFAULT_PLAYBACK_SPEED_RANGE = [0.25, 3] as const
export const DEFAULT_AUTOPLAY = false

export const usePlayer = defineModule(() => {
  const currentMedia = shallowRef<IContentMedia | null>(null)

  const volume = shallowRef(DEFAULT_VOLUME)
  const volumeChange = useIncrementable(volume, { step: DEFAULT_VOLUME_INC_STEP, min: 0, max: 1 })

  const rate = shallowRef(DEFAULT_PLAYBACK_SPEED)
  const rateChange = useIncrementable(rate, {
    step: DEFAULT_PLAYBACK_SPEED_STEP,
    min: DEFAULT_PLAYBACK_SPEED_RANGE[0],
    max: DEFAULT_PLAYBACK_SPEED_RANGE[1],
  })

  const duration = shallowRef(DEFAULT_DURATION)
  const currentTime = shallowRef(DEFAULT_PLAYBACK_SPEED)
  const currentTimeChange = useIncrementable(currentTime, {
    step: 5,
    min: 0,
    max: duration.value,
  })

  const [isPlaying, togglePlaying] = useToggle()
  const [isAutoplay, toggleAutoplay] = useToggle(DEFAULT_AUTOPLAY)
  const [isMuted, toggleMuted] = useToggle()

  const currentTimeSetByPercent = (percent: number) => {
    currentTime.value = Math.round(duration.value * percent)
  }

  const setCurrentMedia = (media: IContentMedia) => {
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
    duration,
  }
})

export function registerPlayerKeybindings(player: ReturnType<typeof usePlayer>) {
  const keyboard = useKeyboard()

  const volumeChangeBezel = () => `${Math.ceil(player.volume * 100)}%`
  const rateChangeBezel = () => `${player.rate}x`

  keyboard.binds({
    'm': {
      pressed: () => player.toggleMuted(),
      bezel: () => player.isMuted ? 'Muted' : 'Unmuted',
    },
    'space': () => player.togglePlaying(),
    'up': {
      pressed: () => player.volumeChange.inc(),
      bezel: volumeChangeBezel,
    },
    'down': {
      pressed: () => player.volumeChange.dec(),
      bezel: volumeChangeBezel,
    },
    'left': {
      pressed: () => player.currentTimeChange.dec(),
      bezel: () => `<< ${player.currentTimeChange.step} sec`,
    },
    'right': {
      pressed: () => player.currentTimeChange.inc(),
      bezel: () => `${player.currentTimeChange.step} sec >>`,
    },
    'shift > .': {
      pressed: () => player.rateChange.inc(),
      bezel: rateChangeBezel,
    },
    'shift > ,': {
      pressed: () => player.rateChange.dec(),
      bezel: rateChangeBezel,
    },
    ...Object.fromEntries(
      timesMap(10, (_, i) => [i, () => player.currentTimeSetByPercent(i / 10)]),
    ),
  })
}

registerPlayerKeybindings(usePlayer())
