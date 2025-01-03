<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn, useDelegatedProps } from '@/shared/utils'
import {
  DropdownMenuSeparator,
  type DropdownMenuSeparatorProps,
} from 'radix-vue'

const props = defineProps<DropdownMenuSeparatorProps & {
  class?: HTMLAttributes['class']
}>()

const delegatedProps = useDelegatedProps(() => props)
</script>

<template>
  <DropdownMenuSeparator v-bind="delegatedProps" :class="cn('-mx-1 my-1 h-px bg-muted', props.class)" />
</template>
