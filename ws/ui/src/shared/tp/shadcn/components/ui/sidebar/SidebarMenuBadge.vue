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
      't:text-sidebar-foreground t:pointer-events-none t:absolute t:right-1 t:flex t:h-5 t:min-w-5 t:items-center t:justify-center t:rounded-md t:px-1 t:text-xs t:font-medium t:tabular-nums t:select-none',
      't:peer-hover/menu-button:text-sidebar-accent-foreground t:peer-data-[active=true]/menu-button:text-sidebar-accent-foreground',
      't:peer-data-[size=sm]/menu-button:top-1',
      't:peer-data-[size=default]/menu-button:top-1.5',
      't:peer-data-[size=lg]/menu-button:top-2.5',
      't:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </div>
</template>
