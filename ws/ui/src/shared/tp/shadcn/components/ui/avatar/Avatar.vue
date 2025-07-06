<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { AvatarRoot } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <AvatarRoot
    data-slot="avatar"
    :class="cn('t:relative t:flex t:size-8 t:shrink-0 t:overflow-hidden t:rounded-full', props.class)"
  >
    <slot />
  </AvatarRoot>
</template>
