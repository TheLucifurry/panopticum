<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="dialog-header"
    :class="cn('t_:flex t_:flex-col t_:gap-2 t_:text-center t_:sm:text-left', props.class)"
  >
    <slot />
  </div>
</template>
