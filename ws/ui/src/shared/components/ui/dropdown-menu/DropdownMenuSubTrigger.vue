<script setup lang="ts">
import type { DropdownMenuSubTriggerProps } from 'radix-vue'
import type { HTMLAttributes } from 'vue'
import { ChevronRightIcon } from '@radix-icons/vue'
import {
  DropdownMenuSubTrigger,

  useForwardProps,
} from 'radix-vue'
import { cn, useDelegatedProps } from '@/shared/utils/shadcn'

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
