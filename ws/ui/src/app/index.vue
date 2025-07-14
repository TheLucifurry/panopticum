<script lang="ts">
import { useColorMode } from '@vueuse/core'
import { defineAsyncComponent, defineComponent, h, watch } from 'vue'
import Desktop from '@/app/layouts/Desktop.vue'
import { useKeyboard } from '@/modules/keyboard'
import { useUiState } from '@/modules/ui'

useKeyboard().init()

const Fullscreen = defineAsyncComponent(() => import('@/app/layouts/Fullscreen.vue'))

export default defineComponent({
  setup() {
    const mode = useColorMode()
    const uis = useUiState()

    watch(() => uis.isFullscreen, v => mode.value = v ? 'dark' : 'light')

    return () => h(!uis.isFullscreen ? Desktop : Fullscreen)
  },
})
</script>
