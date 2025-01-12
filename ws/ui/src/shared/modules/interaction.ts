import type { BezelProvider } from '@/shared/components/custom'
import type { MaybeRef, Ref } from 'vue'
import { whenever } from '@vueuse/core'
import { defineModule } from '@webshrine/vue'
import { isRef, onBeforeUnmount, onScopeDispose, shallowRef, toValue, watch } from 'vue'
import { noop } from 'webshrine'

function usePage() {
  const title = shallowRef('')

  return {
    title,
    defineTitle(caption: MaybeRef<string>) {
      const unwatch = isRef(caption) ? watch(caption, v => title.value = v) : noop
      title.value = toValue(caption)

      onBeforeUnmount(() => {
        title.value = ''
        unwatch()
      })
    },
  }
}

export const useInteraction = defineModule(() => {
  const bezel = shallowRef(null) as Ref<InstanceType<typeof BezelProvider> | null>

  return {
    page: usePage(),
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
