<script setup lang="ts">
import { computed, type HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'
import { Skeleton } from '@/shared/tp/shadcn/components/ui/skeleton'

const props = defineProps<{
  showIcon?: boolean
  class?: HTMLAttributes['class']
}>()

const width = computed(() => {
  return `${Math.floor(Math.random() * 40) + 50}%`
})
</script>

<template>
  <div
    data-slot="sidebar-menu-skeleton"
    data-sidebar="menu-skeleton"
    :class="cn('t_:flex t_:h-8 t_:items-center t_:gap-2 t_:rounded-md t_:px-2', props.class)"
  >
    <Skeleton
      v-if="showIcon"
      class="t_:size-4 t_:rounded-md"
      data-sidebar="menu-skeleton-icon"
    />

    <Skeleton
      class="t_:h-4 t_:max-w-(--skeleton-width) t_:flex-1"
      data-sidebar="menu-skeleton-text"
      :style="{ '--skeleton-width': width }"
    />
  </div>
</template>
