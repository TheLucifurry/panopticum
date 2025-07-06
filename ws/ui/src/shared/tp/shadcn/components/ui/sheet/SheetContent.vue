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
        't_:bg-background t_:data-[state=open]:animate-in t_:data-[state=closed]:animate-out t_:fixed t_:z-50 t_:flex t_:flex-col t_:gap-4 t_:shadow-lg t_:transition t_:ease-in-out t_:data-[state=closed]:duration-300 t_:data-[state=open]:duration-500',
        side === 't_:right'
          && 't_:data-[state=closed]:slide-out-to-right t_:data-[state=open]:slide-in-from-right t_:inset-y-0 t_:right-0 t_:h-full t_:w-3/4 t_:border-l t_:sm:max-w-sm',
        side === 't_:left'
          && 't_:data-[state=closed]:slide-out-to-left t_:data-[state=open]:slide-in-from-left t_:inset-y-0 t_:left-0 t_:h-full t_:w-3/4 t_:border-r t_:sm:max-w-sm',
        side === 't_:top'
          && 't_:data-[state=closed]:slide-out-to-top t_:data-[state=open]:slide-in-from-top t_:inset-x-0 t_:top-0 t_:h-auto t_:border-b',
        side === 't_:bottom'
          && 't_:data-[state=closed]:slide-out-to-bottom t_:data-[state=open]:slide-in-from-bottom t_:inset-x-0 t_:bottom-0 t_:h-auto t_:border-t',
        props.class)"
      v-bind="{ ...forwarded, ...$attrs }"
    >
      <slot />

      <DialogClose
        class="t_:ring-offset-background t_:focus:ring-ring t_:data-[state=open]:bg-secondary t_:absolute t_:top-4 t_:right-4 t_:rounded-xs t_:opacity-70 t_:transition-opacity t_:hover:opacity-100 t_:focus:ring-2 t_:focus:ring-offset-2 t_:focus:outline-hidden t_:disabled:pointer-events-none"
      >
        <X class="t_:size-4" />
        <span class="t_:sr-only">Close</span>
      </DialogClose>
    </DialogContent>
  </DialogPortal>
</template>
