<script setup lang="ts">
import type { SliderRootEmits, SliderRootProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { SliderRange, SliderRoot, SliderThumb, SliderTrack, useForwardPropsEmits } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<SliderRootProps & { class?: HTMLAttributes['class'] }>()
const emits = defineEmits<SliderRootEmits>()

const delegatedProps = reactiveOmit(props, 'class')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <SliderRoot
    v-slot="{ modelValue }"
    data-slot="slider"
    :class="cn(
      't:relative t:flex t:w-full t:touch-none t:items-center t:select-none t:data-[disabled]:opacity-50 t:data-[orientation=vertical]:h-full t:data-[orientation=vertical]:min-h-44 t:data-[orientation=vertical]:w-auto t:data-[orientation=vertical]:flex-col',
      props.class,
    )"
    v-bind="forwarded"
  >
    <SliderTrack
      data-slot="slider-track"
      class="t:bg-muted t:relative t:grow t:overflow-hidden t:rounded-full t:data-[orientation=horizontal]:h-1.5 t:data-[orientation=horizontal]:w-full t:data-[orientation=vertical]:h-full t:data-[orientation=vertical]:w-1.5"
    >
      <SliderRange
        data-slot="slider-range"
        class="t:bg-primary t:absolute t:data-[orientation=horizontal]:h-full t:data-[orientation=vertical]:w-full"
      />
    </SliderTrack>

    <SliderThumb
      v-for="(_, key) in modelValue"
      :key="key"
      data-slot="slider-thumb"
      class="t:border-primary t:bg-background t:ring-ring/50 t:block t:size-4 t:shrink-0 t:rounded-full t:border t:shadow-sm t:transition-[color,box-shadow] t:hover:ring-4 t:focus-visible:ring-4 t:focus-visible:outline-hidden t:disabled:pointer-events-none t:disabled:opacity-50"
    />
  </SliderRoot>
</template>
