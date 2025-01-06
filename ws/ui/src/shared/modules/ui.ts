import { registerUiStateKeybindings } from '@/features/ui'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useToggle } from '@vueuse/core'
import { defineModule } from '@webshrine/vue'
import { shallowRef } from 'vue'
import { cope, debounce } from 'webshrine'

export const useUiState = defineModule(() => {
  const [appWindow] = cope(getCurrentWindow)
  const [isSidebarExpanded, toggleSidebar] = useToggle(true)
  const isFullscreen = shallowRef(false)

  const syncIsFullscreen = () => appWindow?.isFullscreen().then(v => isFullscreen.value = v)
  const toggleFullscreen = () => syncIsFullscreen()?.then(v => appWindow?.setFullscreen(!v))

  // Init
  syncIsFullscreen()
  appWindow?.onResized(debounce(() => {
    syncIsFullscreen()
  }, 16))


  return {
    appWindow,
    isSidebarExpanded,
    toggleSidebar,
    isFullscreen,
    syncIsFullscreen,
    toggleFullscreen,
  }
})

// TODO: implement plugin system for defineModule
registerUiStateKeybindings(useUiState())