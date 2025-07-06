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
    :class="cn('t:flex t:flex-col t:gap-2 t:text-center t:sm:text-left', props.class)"
  >
    <slot />
  </div>
</template>
