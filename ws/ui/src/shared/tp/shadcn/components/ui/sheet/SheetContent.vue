<script setup lang="ts">
import type { DialogContentEmits, DialogContentProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { X } from 'lucide-vue-next'
import {
  DialogClose,
  DialogContent,

  DialogPortal,
  useForwardPropsEmits,
} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'
import SheetOverlay from './SheetOverlay.vue'

interface SheetContentProps extends DialogContentProps {
  class?: HTMLAttributes['class']
  side?: 'top' | 'right' | 'bottom' | 'left'
}

defineOptions({
  inheritAttrs: false,
})

const props = withDefaults(defineProps<SheetContentProps>(), {
  side: 'right',
})
const emits = defineEmits<DialogContentEmits>()

const delegatedProps = reactiveOmit(props, 'class', 'side')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DialogPortal>
    <SheetOverlay />
    <DialogContent
      data-slot="sheet-content"
      :class="cn(
        't:bg-background t:data-[state=open]:animate-in t:data-[state=closed]:animate-out t:fixed t:z-50 t:flex t:flex-col t:gap-4 t:shadow-lg t:transition t:ease-in-out t:data-[state=closed]:duration-300 t:data-[state=open]:duration-500',
        side === 't:right'
          && 't:data-[state=closed]:slide-out-to-right t:data-[state=open]:slide-in-from-right t:inset-y-0 t:right-0 t:h-full t:w-3/4 t:border-l t:sm:max-w-sm',
        side === 't:left'
          && 't:data-[state=closed]:slide-out-to-left t:data-[state=open]:slide-in-from-left t:inset-y-0 t:left-0 t:h-full t:w-3/4 t:border-r t:sm:max-w-sm',
        side === 't:top'
          && 't:data-[state=closed]:slide-out-to-top t:data-[state=open]:slide-in-from-top t:inset-x-0 t:top-0 t:h-auto t:border-b',
        side === 't:bottom'
          && 't:data-[state=closed]:slide-out-to-bottom t:data-[state=open]:slide-in-from-bottom t:inset-x-0 t:bottom-0 t:h-auto t:border-t',
        props.class)"
      v-bind="{ ...forwarded, ...$attrs }"
    >
      <slot />

      <DialogClose
        class="t:ring-offset-background t:focus:ring-ring t:data-[state=open]:bg-secondary t:absolute t:top-4 t:right-4 t:rounded-xs t:opacity-70 t:transition-opacity t:hover:opacity-100 t:focus:ring-2 t:focus:ring-offset-2 t:focus:outline-hidden t:disabled:pointer-events-none"
      >
        <X class="t:size-4" />
        <span class="t:sr-only">Close</span>
      </DialogClose>
    </DialogContent>
  </DialogPortal>
</template>
