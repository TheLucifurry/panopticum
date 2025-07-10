<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="sidebar-group"
    data-sidebar="group"
    :class="cn('t:relative t:flex t:w-full t:min-w-0 t:flex-col t:p-2', props.class)"
  >
    <slot />
  </div>
</template>
