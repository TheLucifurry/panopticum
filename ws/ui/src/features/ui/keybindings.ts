import type { useUiState } from '@/modules/ui'
import { useKeyboard } from '@/modules/keyboard'

export function registerUiStateKeybindings(uis: ReturnType<typeof useUiState>) {
  const keyboard = useKeyboard()

  keyboard.binds({
    f: () => uis.toggleFullscreen(),
    b: () => uis.toggleSidebar(),
  })
}
