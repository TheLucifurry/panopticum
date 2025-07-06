<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="card-footer"
    :class="cn('t_:flex t_:items-center t_:px-6 t_:[.border-t]:pt-6', props.class)"
  >
    <slot />
  </div>
</template>
