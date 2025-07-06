<script setup lang="ts">
import type { PrimitiveProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { Primitive } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<PrimitiveProps & {
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <Primitive
    data-slot="sidebar-group-action"
    data-sidebar="group-action"
    :as="as"
    :as-child="asChild"
    :class="cn(
      't_:text-sidebar-foreground t_:ring-sidebar-ring t_:hover:bg-sidebar-accent t_:hover:text-sidebar-accent-foreground t_:absolute t_:top-3.5 t_:right-3 t_:flex t_:aspect-square t_:w-5 t_:items-center t_:justify-center t_:rounded-md t_:p-0 t_:outline-hidden t_:transition-transform t_:focus-visible:ring-2 t_:[&>svg]:size-4 t_:[&>svg]:shrink-0',
      't_:after:absolute t_:after:-inset-2 t_:md:after:hidden',
      't_:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </Primitive>
</template>
