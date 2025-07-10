<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { Input } from '@/shared/tp/shadcn/components/ui/input'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <Input
    data-slot="sidebar-input"
    data-sidebar="input"
    :class="cn(
      't:bg-background t:h-8 t:w-full t:shadow-none',
      props.class,
    )"
  >
    <slot />
  </Input>
</template>
