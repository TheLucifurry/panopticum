<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="card-action"
    :class="cn('t:col-start-2 t:row-span-2 t:row-start-1 t:self-start t:justify-self-end', props.class)"
  >
    <slot />
  </div>
</template>
