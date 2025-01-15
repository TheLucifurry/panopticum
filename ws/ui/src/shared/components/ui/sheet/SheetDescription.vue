<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn, useDelegatedProps } from '@/shared/utils/shadcn'
import { DialogDescription, type DialogDescriptionProps } from 'radix-vue'

const props = defineProps<DialogDescriptionProps & { class?: HTMLAttributes['class'] }>()

const delegatedProps = useDelegatedProps(() => props)
</script>

<template>
  <DialogDescription
    :class="cn('text-sm text-muted-foreground', props.class)"
    v-bind="delegatedProps"
  >
    <slot />
  </DialogDescription>
</template>
