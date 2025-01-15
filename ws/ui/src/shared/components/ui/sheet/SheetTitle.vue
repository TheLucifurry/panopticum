<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn, useDelegatedProps } from '@/shared/utils/shadcn'
import { DialogTitle, type DialogTitleProps } from 'radix-vue'

const props = defineProps<DialogTitleProps & { class?: HTMLAttributes['class'] }>()

const delegatedProps = useDelegatedProps(() => props)
</script>

<template>
  <DialogTitle
    :class="cn('text-lg font-semibold text-foreground', props.class)"
    v-bind="delegatedProps"
  >
    <slot />
  </DialogTitle>
</template>
