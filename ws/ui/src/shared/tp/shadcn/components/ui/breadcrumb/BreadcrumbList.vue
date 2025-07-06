<script lang="ts" setup>
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <ol
    data-slot="breadcrumb-list"
    :class="cn('t:text-muted-foreground t:flex t:flex-wrap t:items-center t:gap-1.5 t:text-sm t:break-words t:sm:gap-2.5', props.class)"
  >
    <slot />
  </ol>
</template>
