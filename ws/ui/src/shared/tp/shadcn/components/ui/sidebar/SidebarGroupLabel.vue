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
      't:text-sidebar-foreground/70 t:ring-sidebar-ring t:flex t:h-8 t:shrink-0 t:items-center t:rounded-md t:px-2 t:text-xs t:font-medium t:outline-hidden t:transition-[margin,opacity] t:duration-200 t:ease-linear t:focus-visible:ring-2 t:[&>svg]:size-4 t:[&>svg]:shrink-0',
      't:group-data-[collapsible=icon]:-mt-8 t:group-data-[collapsible=icon]:opacity-0',
      props.class)"
  >
    <slot />
  </Primitive>
</template>
