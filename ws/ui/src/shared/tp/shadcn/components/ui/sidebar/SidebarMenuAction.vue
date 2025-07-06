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
      't:text-sidebar-foreground t:ring-sidebar-ring t:hover:bg-sidebar-accent t:hover:text-sidebar-accent-foreground t:peer-hover/menu-button:text-sidebar-accent-foreground t:absolute t:top-1.5 t:right-1 t:flex t:aspect-square t:w-5 t:items-center t:justify-center t:rounded-md t:p-0 t:outline-hidden t:transition-transform t:focus-visible:ring-2 t:[&>svg]:size-4 t:[&>svg]:shrink-0',
      't:after:absolute t:after:-inset-2 t:md:after:hidden',
      't:peer-data-[size=sm]/menu-button:top-1',
      't:peer-data-[size=default]/menu-button:top-1.5',
      't:peer-data-[size=lg]/menu-button:top-2.5',
      't:group-data-[collapsible=icon]:hidden',
      showOnHover
        && 't:peer-data-[active=true]/menu-button:text-sidebar-accent-foreground t:group-focus-within/menu-item:opacity-100 t:group-hover/menu-item:opacity-100 t:data-[state=open]:opacity-100 t:md:opacity-0',
      props.class,
    )"
    :as="as"
    :as-child="asChild"
  >
    <slot />
  </Primitive>
</template>
