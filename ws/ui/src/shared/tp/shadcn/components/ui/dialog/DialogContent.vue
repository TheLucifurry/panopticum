<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { X } from 'lucide-vue-next'
import {
  DialogClose,
  DialogContent,
  type DialogContentEmits,
  type DialogContentProps,
  DialogPortal,
  useForwardPropsEmits,
} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'
import DialogOverlay from './DialogOverlay.vue'

const props = defineProps<DialogContentProps & { class?: HTMLAttributes['class'] }>()
const emits = defineEmits<DialogContentEmits>()

const delegatedProps = reactiveOmit(props, 'class')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DialogPortal>
    <DialogOverlay />
    <DialogContent
      data-slot="dialog-content"
      v-bind="forwarded"
      :class="
        cn(
          't_:bg-background t_:data-[state=open]:animate-in t_:data-[state=closed]:animate-out t_:data-[state=closed]:fade-out-0 t_:data-[state=open]:fade-in-0 t_:data-[state=closed]:zoom-out-95 t_:data-[state=open]:zoom-in-95 t_:fixed t_:top-[50%] t_:left-[50%] t_:z-50 t_:grid t_:w-full t_:max-w-[calc(100%-2rem)] t_:translate-x-[-50%] t_:translate-y-[-50%] t_:gap-4 t_:rounded-lg t_:border t_:p-6 t_:shadow-lg t_:duration-200 t_:sm:max-w-lg',
          props.class,
        )"
    >
      <slot />

      <DialogClose
        class="t_:ring-offset-background t_:focus:ring-ring t_:data-[state=open]:bg-accent t_:data-[state=open]:text-muted-foreground t_:absolute t_:top-4 t_:right-4 t_:rounded-xs t_:opacity-70 t_:transition-opacity t_:hover:opacity-100 t_:focus:ring-2 t_:focus:ring-offset-2 t_:focus:outline-hidden t_:disabled:pointer-events-none t_:[&_svg]:pointer-events-none t_:[&_svg]:shrink-0 t_:[&_svg:not([class*='size-'])]:size-4"
      >
        <X />
        <span class="t_:sr-only">Close</span>
      </DialogClose>
    </DialogContent>
  </DialogPortal>
</template>
