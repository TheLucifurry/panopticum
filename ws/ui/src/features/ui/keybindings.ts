import type { useUiState } from '@/modules'
import { useKeyboard } from '@/modules'

export function registerUiStateKeybindings(uis: ReturnType<typeof useUiState>) {
  const keyboard = useKeyboard()

  keyboard.binds({
    f: () => uis.toggleFullscreen(),
    b: () => uis.toggleSidebar(),
  })
}
