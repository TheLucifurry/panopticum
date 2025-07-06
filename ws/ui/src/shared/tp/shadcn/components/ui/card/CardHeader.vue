<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="card-header"
    :class="cn('t_:@container/card-header t_:grid t_:auto-rows-min t_:grid-rows-[auto_auto] t_:items-start t_:gap-1.5 t_:px-6 t_:has-data-[slot=card-action]:grid-cols-[1fr_auto] t_:[.border-b]:pb-6', props.class)"
  >
    <slot />
  </div>
</template>
