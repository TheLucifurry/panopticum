<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()
</script>

<template>
  <div
    data-slot="dialog-footer"
    :class="cn('t:flex t:flex-col-reverse t:gap-2 t:sm:flex-row t:sm:justify-end', props.class)"
  >
    <slot />
  </div>
</template>
