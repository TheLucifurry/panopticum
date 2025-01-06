import type { useUiState } from '@/shared/modules'
import { useKeyboard } from '@/shared/modules'

export function registerUiStateKeybindings(uis: ReturnType<typeof useUiState>) {
  const keyboard = useKeyboard()

  keyboard.binds({
    f: () => uis.toggleFullscreen(),
    b: () => uis.toggleSidebar(),
  })
}
