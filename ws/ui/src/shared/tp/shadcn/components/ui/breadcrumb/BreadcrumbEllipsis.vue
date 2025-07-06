<script lang="ts" setup>
import type { HTMLAttributes } from 'vue'
import { MoreHorizontal } from 'lucide-vue-next'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <span
    data-slot="breadcrumb-ellipsis"
    role="presentation"
    aria-hidden="true"
    :class="cn('t:flex t:size-9 t:items-center t:justify-center', props.class)"
  >
    <slot>
      <MoreHorizontal class="t:size-4" />
    </slot>
    <span class="t:sr-only">More</span>
  </span>
</template>
