<script setup lang="ts">
import type { DialogTitleProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { DialogTitle } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DialogTitleProps & { class?: HTMLAttributes['class'] }>()

const delegatedProps = reactiveOmit(props, 'class')
</script>

<template>
  <DialogTitle
    data-slot="sheet-title"
    :class="cn('t:text-foreground t:font-semibold', props.class)"
    v-bind="delegatedProps"
  >
    <slot />
  </DialogTitle>
</template>
