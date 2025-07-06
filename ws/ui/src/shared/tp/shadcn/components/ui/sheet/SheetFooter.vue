<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()
</script>

<template>
  <div
    data-slot="sheet-footer"
    :class="cn('t:mt-auto t:flex t:flex-col t:gap-2 t:p-4', props.class)
    "
  >
    <slot />
  </div>
</template>
