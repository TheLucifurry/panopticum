<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { X } from 'lucide-vue-next'
import {
  DialogClose,
  DialogContent,
  type DialogContentEmits,
  type DialogContentProps,
  DialogOverlay,
  DialogPortal,
  useForwardPropsEmits,
} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DialogContentProps & { class?: HTMLAttributes['class'] }>()
const emits = defineEmits<DialogContentEmits>()

const delegatedProps = reactiveOmit(props, 'class')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DialogPortal>
    <DialogOverlay
      class="t_:fixed t_:inset-0 t_:z-50 t_:grid t_:place-items-center t_:overflow-y-auto t_:bg-black/80 t_: t_:data-[state=open]:animate-in t_:data-[state=closed]:animate-out t_:data-[state=closed]:fade-out-0 t_:data-[state=open]:fade-in-0"
    >
      <DialogContent
        :class="
          cn(
            't_:relative t_:z-50 t_:grid t_:w-full t_:max-w-lg t_:my-8 t_:gap-4 t_:border t_:border-border t_:bg-background t_:p-6 t_:shadow-lg t_:duration-200 t_:sm:rounded-lg t_:md:w-full',
            props.class,
          )
        "
        v-bind="forwarded"
        @pointer-down-outside="(event) => {
          const originalEvent = event.detail.originalEvent;
          const target = originalEvent.target as HTMLElement;
          if (originalEvent.offsetX > target.clientWidth || originalEvent.offsetY > target.clientHeight) {
            event.preventDefault();
          }
        }"
      >
        <slot />

        <DialogClose
          class="t_:absolute t_:top-4 t_:right-4 t_:p-0.5 t_:transition-colors t_:rounded-md t_:hover:bg-secondary"
        >
          <X class="t_:w-4 t_:h-4" />
          <span class="t_:sr-only">Close</span>
        </DialogClose>
      </DialogContent>
    </DialogOverlay>
  </DialogPortal>
</template>
