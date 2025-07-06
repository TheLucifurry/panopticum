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
    :class="cn('t_:relative t_:flex t_:size-8 t_:shrink-0 t_:overflow-hidden t_:rounded-full', props.class)"
  >
    <slot />
  </AvatarRoot>
</template>
