<script setup lang="ts">
import type { DropdownMenuSeparatorProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  DropdownMenuSeparator,

} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DropdownMenuSeparatorProps & {
  class?: HTMLAttributes['class']
}>()

const delegatedProps = reactiveOmit(props, 'class')
</script>

<template>
  <DropdownMenuSeparator
    data-slot="dropdown-menu-separator"
    v-bind="delegatedProps"
    :class="cn('t_:bg-border t_:-mx-1 t_:my-1 t_:h-px', props.class)"
  />
</template>
