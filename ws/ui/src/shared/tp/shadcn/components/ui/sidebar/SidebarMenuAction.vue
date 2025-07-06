<script setup lang="ts">
import type { PrimitiveProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { Primitive } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = withDefaults(defineProps<PrimitiveProps & {
  showOnHover?: boolean
  class?: HTMLAttributes['class']
}>(), {
  as: 'button',
})
</script>

<template>
  <Primitive
    data-slot="sidebar-menu-action"
    data-sidebar="menu-action"
    :class="cn(
      't_:text-sidebar-foreground t_:ring-sidebar-ring t_:hover:bg-sidebar-accent t_:hover:text-sidebar-accent-foreground t_:peer-hover/menu-button:text-sidebar-accent-foreground t_:absolute t_:top-1.5 t_:right-1 t_:flex t_:aspect-square t_:w-5 t_:items-center t_:justify-center t_:rounded-md t_:p-0 t_:outline-hidden t_:transition-transform t_:focus-visible:ring-2 t_:[&>svg]:size-4 t_:[&>svg]:shrink-0',
      't_:after:absolute t_:after:-inset-2 t_:md:after:hidden',
      't_:peer-data-[size=sm]/menu-button:top-1',
      't_:peer-data-[size=default]/menu-button:top-1.5',
      't_:peer-data-[size=lg]/menu-button:top-2.5',
      't_:group-data-[collapsible=icon]:hidden',
      showOnHover
        && 't_:peer-data-[active=true]/menu-button:text-sidebar-accent-foreground t_:group-focus-within/menu-item:opacity-100 t_:group-hover/menu-item:opacity-100 t_:data-[state=open]:opacity-100 t_:md:opacity-0',
      props.class,
    )"
    :as="as"
    :as-child="asChild"
  >
    <slot />
  </Primitive>
</template>
