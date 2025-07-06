<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <p
    data-slot="card-description"
    :class="cn('t_:text-muted-foreground t_:text-sm', props.class)"
  >
    <slot />
  </p>
</template>
