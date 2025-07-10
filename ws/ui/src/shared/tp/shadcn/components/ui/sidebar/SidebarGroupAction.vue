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
      't:text-sidebar-foreground t:ring-sidebar-ring t:hover:bg-sidebar-accent t:hover:text-sidebar-accent-foreground t:absolute t:top-3.5 t:right-3 t:flex t:aspect-square t:w-5 t:items-center t:justify-center t:rounded-md t:p-0 t:outline-hidden t:transition-transform t:focus-visible:ring-2 t:[&>svg]:size-4 t:[&>svg]:shrink-0',
      't:after:absolute t:after:-inset-2 t:md:after:hidden',
      't:group-data-[collapsible=icon]:hidden',
      props.class,
    )"
  >
    <slot />
  </Primitive>
</template>
