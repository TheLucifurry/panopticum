<script setup lang="ts">
import type { PrimitiveProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { Primitive } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = withDefaults(defineProps<PrimitiveProps & {
  size?: 'sm' | 'md'
  isActive?: boolean
  class?: HTMLAttributes['class']
}>(), {
  as: 'a',
  size: 'md',
})
</script>

<template>
  <Primitive
    data-slot="sidebar-menu-sub-button"
    data-sidebar="menu-sub-button"
    :as="as"
    :as-child="asChild"
    :data-size="size"
    :data-active="isActive"
    :class="cn(
      't_:text-sidebar-foreground t_:ring-sidebar-ring t_:hover:bg-sidebar-accent t_:hover:text-sidebar-accent-foreground t_:active:bg-sidebar-accent t_:active:text-sidebar-accent-foreground t_:[&>svg]:text-sidebar-accent-foreground t_:flex t_:h-7 t_:min-w-0 t_:-translate-x-px t_:items-center t_:gap-2 t_:overflow-hidden t_:rounded-md t_:px-2 t_:outline-hidden t_:focus-visible:ring-2 t_:disabled:pointer-events-none t_:disabled:opacity-50 t_:aria-disabled:pointer-events-none t_:aria-disabled:opacity-50 t_:[&>span:last-child]:truncate t_:[&>svg]:size-4 t_:[&>svg]:shrink-0',
      't_:data-[active=true]:bg-sidebar-accent t_:data-[active=true]:text-sidebar-accent-foreground',
      size === 't_:sm' && 't_:text-xs',
      size === 't_:md' && 't_:text-sm',
      't_:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </Primitive>
</template>
