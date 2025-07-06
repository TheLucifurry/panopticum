<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="sidebar-group"
    data-sidebar="group"
    :class="cn('t_:relative t_:flex t_:w-full t_:min-w-0 t_:flex-col t_:p-2', props.class)"
  >
    <slot />
  </div>
</template>
