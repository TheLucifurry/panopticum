<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    data-slot="card"
    :class="
      cn(
        't_:bg-card t_:text-card-foreground t_:flex t_:flex-col t_:gap-6 t_:rounded-xl t_:border t_:py-6 t_:shadow-sm',
        props.class,
      )
    "
  >
    <slot />
  </div>
</template>
