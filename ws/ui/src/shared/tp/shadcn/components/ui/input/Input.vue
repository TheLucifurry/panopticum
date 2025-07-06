<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { useVModel } from '@vueuse/core'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  defaultValue?: string | number
  modelValue?: string | number
  class?: HTMLAttributes['class']
}>()

const emits = defineEmits<{
  (e: 'update:modelValue', payload: string | number): void
}>()

const modelValue = useVModel(props, 'modelValue', emits, {
  passive: true,
  defaultValue: props.defaultValue,
})
</script>

<template>
  <input
    v-model="modelValue"
    data-slot="input"
    :class="cn(
      't:file:text-foreground t:placeholder:text-muted-foreground t:selection:bg-primary t:selection:text-primary-foreground t:dark:bg-input/30 t:border-input t:flex t:h-9 t:w-full t:min-w-0 t:rounded-md t:border t:bg-transparent t:px-3 t:py-1 t:text-base t:shadow-xs t:transition-[color,box-shadow] t:outline-none t:file:inline-flex t:file:h-7 t:file:border-0 t:file:bg-transparent t:file:text-sm t:file:font-medium t:disabled:pointer-events-none t:disabled:cursor-not-allowed t:disabled:opacity-50 t:md:text-sm',
      't:focus-visible:border-ring t:focus-visible:ring-ring/50 t:focus-visible:ring-[3px]',
      't:aria-invalid:ring-destructive/20 t:dark:aria-invalid:ring-destructive/40 t:aria-invalid:border-destructive',
      props.class,
    )"
  >
</template>
