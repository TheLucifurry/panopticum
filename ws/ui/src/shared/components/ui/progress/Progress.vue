<script setup lang="ts">
import { cn, useDelegatedProps } from '@/shared/utils/shadcn'
import {
  ProgressIndicator,
  ProgressRoot,
  type ProgressRootProps,
} from 'radix-vue'
import { computed, type HTMLAttributes } from 'vue'

const props = withDefaults(
  defineProps<ProgressRootProps & { class?: HTMLAttributes['class'] }>(),
  {
    modelValue: 0,
  },
)

const progressPercent = computed(() => (props?.modelValue || 0) / (props?.max || 1))

const delegatedProps = useDelegatedProps(() => props)
</script>

<template>
  <ProgressRoot
    v-bind="delegatedProps"
    :class="
      cn(
        'relative h-2 w-full overflow-hidden rounded-full bg-primary/20',
        props.class,
      )
    "
  >
    <ProgressIndicator
      class="h-full w-full flex-1 bg-primary transition-all"
      :style="`transform: translateX(-${(1 - progressPercent) * 100}%);`"
    />
  </ProgressRoot>
</template>
