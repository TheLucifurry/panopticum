import type { usePlayer } from '@/shared/modules'
import { useKeyboard } from '@/shared/modules'
import { timesMap } from 'webshrine'

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
