<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'
import { useSidebar } from './utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()

const { toggleSidebar } = useSidebar()
</script>

<template>
  <button
    data-sidebar="rail"
    data-slot="sidebar-rail"
    aria-label="Toggle Sidebar"
    :tabindex="-1"
    title="Toggle Sidebar"
    :class="cn(
      't:hover:after:bg-sidebar-border t:absolute t:inset-y-0 t:z-20 t:hidden t:w-4 t:-translate-x-1/2 t:transition-all t:ease-linear t:group-data-[side=left]:-right-4 t:group-data-[side=right]:left-0 t:after:absolute t:after:inset-y-0 t:after:left-1/2 t:after:w-[2px] t:sm:flex',
      't:in-data-[side=left]:cursor-w-resize t:in-data-[side=right]:cursor-e-resize',
      't:[[data-side=left][data-state=collapsed]_&]:cursor-e-resize t:[[data-side=right][data-state=collapsed]_&]:cursor-w-resize',
      't:hover:group-data-[collapsible=offcanvas]:bg-sidebar t:group-data-[collapsible=offcanvas]:translate-x-0 t:group-data-[collapsible=offcanvas]:after:left-full',
      't:[[data-side=left][data-collapsible=offcanvas]_&]:-right-2',
      't:[[data-side=right][data-collapsible=offcanvas]_&]:-left-2',
      props.class,
    )"
    @click="toggleSidebar"
  >
    <slot />
  </button>
</template>
