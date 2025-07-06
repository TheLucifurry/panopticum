<script setup lang="ts">
import type { DropdownMenuSubTriggerProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { ChevronRight } from 'lucide-vue-next'
import {
  DropdownMenuSubTrigger,

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
      't:focus:bg-accent t:focus:text-accent-foreground t:data-[state=open]:bg-accent t:data-[state=open]:text-accent-foreground t:flex t:cursor-default t:items-center t:rounded-sm t:px-2 t:py-1.5 t:text-sm t:outline-hidden t:select-none t:data-[inset]:pl-8',
      props.class,
    )"
  >
    <slot />
    <ChevronRight class="t:ml-auto t:size-4" />
  </DropdownMenuSubTrigger>
</template>
