import type { BezelProvider } from '@/shared/components/custom'
import { whenever } from '@vueuse/core'
import { defineModule } from '@webshrine/vue'
import { onScopeDispose, type Ref, shallowRef } from 'vue'

export const useInteraction = defineModule(() => {
  const bezel = shallowRef(null) as Ref<InstanceType<typeof BezelProvider> | null>

  return {
    bezel: {
      connect(ref: Ref<InstanceType<typeof BezelProvider> | null>) {
        whenever(ref, () => bezel.value = ref.value)
        onScopeDispose(() => bezel.value = null)
      },
      show(content: any) {
        bezel.value?.show(content)
      },
    },
  }
})
