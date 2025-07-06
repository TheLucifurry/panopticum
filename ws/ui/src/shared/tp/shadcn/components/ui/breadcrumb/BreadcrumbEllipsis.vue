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
    :class="cn('t_:flex t_:size-9 t_:items-center t_:justify-center', props.class)"
  >
    <slot>
      <MoreHorizontal class="t_:size-4" />
    </slot>
    <span class="t_:sr-only">More</span>
  </span>
</template>
