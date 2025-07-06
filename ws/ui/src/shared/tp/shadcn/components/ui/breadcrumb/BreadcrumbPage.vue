<script lang="ts" setup>
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <span
    data-slot="breadcrumb-page"
    role="link"
    aria-disabled="true"
    aria-current="page"
    :class="cn('t_:text-foreground t_:font-normal', props.class)"
  >
    <slot />
  </span>
</template>
