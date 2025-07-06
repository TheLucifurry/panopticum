<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="sidebar-menu-badge"
    data-sidebar="menu-badge"
    :class="cn(
      't_:text-sidebar-foreground t_:pointer-events-none t_:absolute t_:right-1 t_:flex t_:h-5 t_:min-w-5 t_:items-center t_:justify-center t_:rounded-md t_:px-1 t_:text-xs t_:font-medium t_:tabular-nums t_:select-none',
      't_:peer-hover/menu-button:text-sidebar-accent-foreground t_:peer-data-[active=true]/menu-button:text-sidebar-accent-foreground',
      't_:peer-data-[size=sm]/menu-button:top-1',
      't_:peer-data-[size=default]/menu-button:top-1.5',
      't_:peer-data-[size=lg]/menu-button:top-2.5',
      't_:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </div>
</template>
