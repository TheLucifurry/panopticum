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
    :class="cn('t:flex t:items-center t:px-6 t:[.border-t]:pt-6', props.class)"
  >
    <slot />
  </div>
</template>
