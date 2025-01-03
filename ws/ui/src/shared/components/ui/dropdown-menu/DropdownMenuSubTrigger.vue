<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn, useDelegatedProps } from '@/shared/utils'
import { ChevronRightIcon } from '@radix-icons/vue'
import {
  DropdownMenuSubTrigger,
  type DropdownMenuSubTriggerProps,
  useForwardProps,
} from 'radix-vue'

const props = defineProps<DropdownMenuSubTriggerProps & { class?: HTMLAttributes['class'] }>()

const delegatedProps = useDelegatedProps(() => props)

const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <DropdownMenuSubTrigger
    v-bind="forwardedProps"
    :class="cn(
      'flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent data-[state=open]:bg-accent',
      props.class,
    )"
  >
    <slot />
    <ChevronRightIcon class="ml-auto h-4 w-4" />
  </DropdownMenuSubTrigger>
</template>
