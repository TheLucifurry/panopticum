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
    :class="cn('t_:col-start-2 t_:row-span-2 t_:row-start-1 t_:self-start t_:justify-self-end', props.class)"
  >
    <slot />
  </div>
</template>
