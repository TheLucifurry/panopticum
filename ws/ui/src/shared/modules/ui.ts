import { getCurrentWindow } from '@tauri-apps/api/window'
import { defineComposable } from '@webshrine/vue'
import { shallowRef } from 'vue'
import { cope, debounce } from 'webshrine'

export const useUiState = defineComposable({
  singleton: true,
  flat: true,
  setup() {
    const [appWindow] = cope(getCurrentWindow)
    const isSidebarExpanded = shallowRef(true)
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
      isFullscreen,
      syncIsFullscreen,
      toggleFullscreen,
    }
  },
})
