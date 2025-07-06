<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()
</script>

<template>
  <div
    data-slot="dialog-footer"
    :class="cn('t_:flex t_:flex-col-reverse t_:gap-2 t_:sm:flex-row t_:sm:justify-end', props.class)"
  >
    <slot />
  </div>
</template>
