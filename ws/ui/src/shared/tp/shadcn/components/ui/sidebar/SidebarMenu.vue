<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <ul
    data-slot="sidebar-menu"
    data-sidebar="menu"
    :class="cn('t:flex t:w-full t:min-w-0 t:flex-col t:gap-1', props.class)"
  >
    <slot />
  </ul>
</template>
