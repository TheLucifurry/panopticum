<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <ul
    data-slot="sidebar-menu-sub"
    data-sidebar="menu-badge"
    :class="cn(
      't:border-sidebar-border t:mx-3.5 t:flex t:min-w-0 t:translate-x-px t:flex-col t:gap-1 t:border-l t:px-2.5 t:py-0.5',
      't:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </ul>
</template>
