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
      't:text-sidebar-foreground t:ring-sidebar-ring t:hover:bg-sidebar-accent t:hover:text-sidebar-accent-foreground t:active:bg-sidebar-accent t:active:text-sidebar-accent-foreground t:[&>svg]:text-sidebar-accent-foreground t:flex t:h-7 t:min-w-0 t:-translate-x-px t:items-center t:gap-2 t:overflow-hidden t:rounded-md t:px-2 t:outline-hidden t:focus-visible:ring-2 t:disabled:pointer-events-none t:disabled:opacity-50 t:aria-disabled:pointer-events-none t:aria-disabled:opacity-50 t:[&>span:last-child]:truncate t:[&>svg]:size-4 t:[&>svg]:shrink-0',
      't:data-[active=true]:bg-sidebar-accent t:data-[active=true]:text-sidebar-accent-foreground',
      size === 't:sm' && 't:text-xs',
      size === 't:md' && 't:text-sm',
      't:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </Primitive>
</template>
