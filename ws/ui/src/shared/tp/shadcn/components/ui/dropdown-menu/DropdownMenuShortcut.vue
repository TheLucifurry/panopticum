<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <span
    data-slot="dropdown-menu-shortcut"
    :class="cn('t:text-muted-foreground t:ml-auto t:text-xs t:tracking-widest', props.class)"
  >
    <slot />
  </span>
</template>
