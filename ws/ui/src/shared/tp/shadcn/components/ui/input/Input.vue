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
      't_:file:text-foreground t_:placeholder:text-muted-foreground t_:selection:bg-primary t_:selection:text-primary-foreground t_:dark:bg-input/30 t_:border-input t_:flex t_:h-9 t_:w-full t_:min-w-0 t_:rounded-md t_:border t_:bg-transparent t_:px-3 t_:py-1 t_:text-base t_:shadow-xs t_:transition-[color,box-shadow] t_:outline-none t_:file:inline-flex t_:file:h-7 t_:file:border-0 t_:file:bg-transparent t_:file:text-sm t_:file:font-medium t_:disabled:pointer-events-none t_:disabled:cursor-not-allowed t_:disabled:opacity-50 t_:md:text-sm',
      't_:focus-visible:border-ring t_:focus-visible:ring-ring/50 t_:focus-visible:ring-[3px]',
      't_:aria-invalid:ring-destructive/20 t_:dark:aria-invalid:ring-destructive/40 t_:aria-invalid:border-destructive',
      props.class,
    )"
  >
</template>
