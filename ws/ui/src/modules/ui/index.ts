import { getCurrentWindow } from '@tauri-apps/api/window'
import { useToggle } from '@vueuse/core'
import { defineModule } from '@webshrine/vue'
import { shallowRef } from 'vue'
import { cope, debounce } from 'webshrine'
import { useKeyboard } from '@/modules/keyboard'

export const useUiState = defineModule(() => {
  const [appWindow] = cope(getCurrentWindow)
  const [isSidebarExpanded, toggleSidebar] = useToggle()
  const isFullscreen = shallowRef(false)
  const keyboard = useKeyboard()

  const syncIsFullscreen = () => appWindow?.isFullscreen().then(v => isFullscreen.value = v)
  const toggleFullscreen = () => syncIsFullscreen()?.then(v => appWindow?.setFullscreen(!v))

  // Init
  syncIsFullscreen()
  appWindow?.onResized(debounce(() => {
    syncIsFullscreen()
  }, 16))

  keyboard.binds({
    f: () => toggleFullscreen(),
    b: () => toggleSidebar(),
  })

  return {
    appWindow,
    isSidebarExpanded,
    toggleSidebar,
    isFullscreen,
    syncIsFullscreen,
    toggleFullscreen,
  }
})
