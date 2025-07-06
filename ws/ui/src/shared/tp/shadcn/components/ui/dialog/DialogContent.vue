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
          't:bg-background t:data-[state=open]:animate-in t:data-[state=closed]:animate-out t:data-[state=closed]:fade-out-0 t:data-[state=open]:fade-in-0 t:data-[state=closed]:zoom-out-95 t:data-[state=open]:zoom-in-95 t:fixed t:top-[50%] t:left-[50%] t:z-50 t:grid t:w-full t:max-w-[calc(100%-2rem)] t:translate-x-[-50%] t:translate-y-[-50%] t:gap-4 t:rounded-lg t:border t:p-6 t:shadow-lg t:duration-200 t:sm:max-w-lg',
          props.class,
        )"
    >
      <slot />

      <DialogClose
        class="t:ring-offset-background t:focus:ring-ring t:data-[state=open]:bg-accent t:data-[state=open]:text-muted-foreground t:absolute t:top-4 t:right-4 t:rounded-xs t:opacity-70 t:transition-opacity t:hover:opacity-100 t:focus:ring-2 t:focus:ring-offset-2 t:focus:outline-hidden t:disabled:pointer-events-none t:[&_svg]:pointer-events-none t:[&_svg]:shrink-0 t:[&_svg:not([class*='size-'])]:size-4"
      >
        <X />
        <span class="t:sr-only">Close</span>
      </DialogClose>
    </DialogContent>
  </DialogPortal>
</template>
