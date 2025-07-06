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
    data-slot="sidebar-group-label"
    data-sidebar="group-label"
    :as="as"
    :as-child="asChild"
    :class="cn(
      't_:text-sidebar-foreground/70 t_:ring-sidebar-ring t_:flex t_:h-8 t_:shrink-0 t_:items-center t_:rounded-md t_:px-2 t_:text-xs t_:font-medium t_:outline-hidden t_:transition-[margin,opacity] t_:duration-200 t_:ease-linear t_:focus-visible:ring-2 t_:[&>svg]:size-4 t_:[&>svg]:shrink-0',
      't_:group-data-[collapsible=icon]:-mt-8 t_:group-data-[collapsible=icon]:opacity-0',
      props.class)"
  >
    <slot />
  </Primitive>
</template>
