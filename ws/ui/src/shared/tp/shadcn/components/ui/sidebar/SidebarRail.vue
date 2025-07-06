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
      't_:hover:after:bg-sidebar-border t_:absolute t_:inset-y-0 t_:z-20 t_:hidden t_:w-4 t_:-translate-x-1/2 t_:transition-all t_:ease-linear t_:group-data-[side=left]:-right-4 t_:group-data-[side=right]:left-0 t_:after:absolute t_:after:inset-y-0 t_:after:left-1/2 t_:after:w-[2px] t_:sm:flex',
      't_:in-data-[side=left]:cursor-w-resize t_:in-data-[side=right]:cursor-e-resize',
      't_:[[data-side=left][data-state=collapsed]_&]:cursor-e-resize t_:[[data-side=right][data-state=collapsed]_&]:cursor-w-resize',
      't_:hover:group-data-[collapsible=offcanvas]:bg-sidebar t_:group-data-[collapsible=offcanvas]:translate-x-0 t_:group-data-[collapsible=offcanvas]:after:left-full',
      't_:[[data-side=left][data-collapsible=offcanvas]_&]:-right-2',
      't_:[[data-side=right][data-collapsible=offcanvas]_&]:-left-2',
      props.class,
    )"
    @click="toggleSidebar"
  >
    <slot />
  </button>
</template>
