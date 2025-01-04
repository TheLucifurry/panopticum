<script lang="ts">
import Desktop from '@/app/layouts/Desktop.vue'
import { useUiState } from '@/shared/modules'
import { defineAsyncComponent, defineComponent, h } from 'vue'
import { useKeyboard } from '../shared/modules/keyboard'

useKeyboard().init()

const Fullscreen = defineAsyncComponent(() => import('@/app/layouts/Fullscreen.vue'))

export default defineComponent({
  setup() {
    const uis = useUiState()

    return () => h(!uis.isFullscreen ? Desktop : Fullscreen)
  },
})
</script>
