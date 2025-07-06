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
      't_:border-sidebar-border t_:mx-3.5 t_:flex t_:min-w-0 t_:translate-x-px t_:flex-col t_:gap-1 t_:border-l t_:px-2.5 t_:py-0.5',
      't_:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </ul>
</template>
