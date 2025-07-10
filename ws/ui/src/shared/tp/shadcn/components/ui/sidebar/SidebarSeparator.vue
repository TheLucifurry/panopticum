<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { Separator } from '@/shared/tp/shadcn/components/ui/separator'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <Separator
    data-slot="sidebar-separator"
    data-sidebar="separator"
    :class="cn('t:bg-sidebar-border t:mx-2 t:w-auto', props.class)"
  >
    <slot />
  </Separator>
</template>
