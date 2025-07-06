<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()
</script>

<template>
  <div
    data-slot="sheet-footer"
    :class="cn('t_:mt-auto t_:flex t_:flex-col t_:gap-2 t_:p-4', props.class)
    "
  >
    <slot />
  </div>
</template>
