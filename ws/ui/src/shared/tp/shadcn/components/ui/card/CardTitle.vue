<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <h3
    data-slot="card-title"
    :class="cn('t:leading-none t:font-semibold', props.class)"
  >
    <slot />
  </h3>
</template>
