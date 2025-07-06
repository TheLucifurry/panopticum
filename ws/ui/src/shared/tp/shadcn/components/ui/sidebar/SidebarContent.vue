<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="sidebar-content"
    data-sidebar="content"
    :class="cn('t_:flex t_:min-h-0 t_:flex-1 t_:flex-col t_:gap-2 t_:overflow-auto t_:group-data-[collapsible=icon]:overflow-hidden', props.class)"
  >
    <slot />
  </div>
</template>
