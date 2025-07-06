<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { DropdownMenuLabel, type DropdownMenuLabelProps, useForwardProps } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DropdownMenuLabelProps & { class?: HTMLAttributes['class'], inset?: boolean }>()

const delegatedProps = reactiveOmit(props, 'class', 'inset')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <DropdownMenuLabel
    data-slot="dropdown-menu-label"
    :data-inset="inset ? '' : undefined"
    v-bind="forwardedProps"
    :class="cn('t_:px-2 t_:py-1.5 t_:text-sm t_:font-medium t_:data-[inset]:pl-8', props.class)"
  >
    <slot />
  </DropdownMenuLabel>
</template>
