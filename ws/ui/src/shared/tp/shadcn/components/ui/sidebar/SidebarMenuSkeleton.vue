<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { computed } from 'vue'
import { Skeleton } from '@/shared/tp/shadcn/components/ui/skeleton'
import { cn } from '@/shared/tp/shadcn/lib/utils'

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
    :class="cn('t:flex t:h-8 t:items-center t:gap-2 t:rounded-md t:px-2', props.class)"
  >
    <Skeleton
      v-if="showIcon"
      class="t:size-4 t:rounded-md"
      data-sidebar="menu-skeleton-icon"
    />

    <Skeleton
      class="t:h-4 t:max-w-(--skeleton-width) t:flex-1"
      data-sidebar="menu-skeleton-text"
      :style="{ '--skeleton-width': width }"
    />
  </div>
</template>
