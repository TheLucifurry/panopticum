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
    :class="cn('t_:text-muted-foreground t_:flex t_:flex-wrap t_:items-center t_:gap-1.5 t_:text-sm t_:break-words t_:sm:gap-2.5', props.class)"
  >
    <slot />
  </ol>
</template>
