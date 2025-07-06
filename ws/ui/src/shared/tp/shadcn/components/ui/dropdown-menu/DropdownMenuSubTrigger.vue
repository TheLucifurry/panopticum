<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { ChevronRight } from 'lucide-vue-next'
import {
  DropdownMenuSubTrigger,
  type DropdownMenuSubTriggerProps,
  useForwardProps,
} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DropdownMenuSubTriggerProps & { class?: HTMLAttributes['class'], inset?: boolean }>()

const delegatedProps = reactiveOmit(props, 'class', 'inset')
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <DropdownMenuSubTrigger
    data-slot="dropdown-menu-sub-trigger"
    v-bind="forwardedProps"
    :class="cn(
      't_:focus:bg-accent t_:focus:text-accent-foreground t_:data-[state=open]:bg-accent t_:data-[state=open]:text-accent-foreground t_:flex t_:cursor-default t_:items-center t_:rounded-sm t_:px-2 t_:py-1.5 t_:text-sm t_:outline-hidden t_:select-none t_:data-[inset]:pl-8',
      props.class,
    )"
  >
    <slot />
    <ChevronRight class="t_:ml-auto t_:size-4" />
  </DropdownMenuSubTrigger>
</template>
