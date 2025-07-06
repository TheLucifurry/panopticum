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
        't:bg-card t:text-card-foreground t:flex t:flex-col t:gap-6 t:rounded-xl t:border t:py-6 t:shadow-sm',
        props.class,
      )
    "
  >
    <slot />
  </div>
</template>
