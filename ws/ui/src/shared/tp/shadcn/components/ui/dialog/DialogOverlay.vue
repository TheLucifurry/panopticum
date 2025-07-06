<script setup lang="ts">
import type { DialogOverlayProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { DialogOverlay } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DialogOverlayProps & { class?: HTMLAttributes['class'] }>()

const delegatedProps = reactiveOmit(props, 'class')
</script>

<template>
  <DialogOverlay
    data-slot="dialog-overlay"
    v-bind="delegatedProps"
    :class="cn('t_:data-[state=open]:animate-in t_:data-[state=closed]:animate-out t_:data-[state=closed]:fade-out-0 t_:data-[state=open]:fade-in-0 t_:fixed t_:inset-0 t_:z-50 t_:bg-black/80', props.class)"
  >
    <slot />
  </DialogOverlay>
</template>
