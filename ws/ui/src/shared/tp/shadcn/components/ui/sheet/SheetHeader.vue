<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()
</script>

<template>
  <div
    data-slot="sheet-header"
    :class="cn('t_:flex t_:flex-col t_:gap-1.5 t_:p-4', props.class)"
  >
    <slot />
  </div>
</template>
