<script lang="ts" setup>
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <li
    data-slot="breadcrumb-item"
    :class="cn('t:inline-flex t:items-center t:gap-1.5', props.class)"
  >
    <slot />
  </li>
</template>
