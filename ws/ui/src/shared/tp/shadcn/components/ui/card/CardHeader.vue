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
    :class="cn('t:@container/card-header t:grid t:auto-rows-min t:grid-rows-[auto_auto] t:items-start t:gap-1.5 t:px-6 t:has-data-[slot=card-action]:grid-cols-[1fr_auto] t:[.border-b]:pb-6', props.class)"
  >
    <slot />
  </div>
</template>
