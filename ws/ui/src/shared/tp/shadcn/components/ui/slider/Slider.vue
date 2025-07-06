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
      't_:relative t_:flex t_:w-full t_:touch-none t_:items-center t_:select-none t_:data-[disabled]:opacity-50 t_:data-[orientation=vertical]:h-full t_:data-[orientation=vertical]:min-h-44 t_:data-[orientation=vertical]:w-auto t_:data-[orientation=vertical]:flex-col',
      props.class,
    )"
    v-bind="forwarded"
  >
    <SliderTrack
      data-slot="slider-track"
      class="t_:bg-muted t_:relative t_:grow t_:overflow-hidden t_:rounded-full t_:data-[orientation=horizontal]:h-1.5 t_:data-[orientation=horizontal]:w-full t_:data-[orientation=vertical]:h-full t_:data-[orientation=vertical]:w-1.5"
    >
      <SliderRange
        data-slot="slider-range"
        class="t_:bg-primary t_:absolute t_:data-[orientation=horizontal]:h-full t_:data-[orientation=vertical]:w-full"
      />
    </SliderTrack>

    <SliderThumb
      v-for="(_, key) in modelValue"
      :key="key"
      data-slot="slider-thumb"
      class="t_:border-primary t_:bg-background t_:ring-ring/50 t_:block t_:size-4 t_:shrink-0 t_:rounded-full t_:border t_:shadow-sm t_:transition-[color,box-shadow] t_:hover:ring-4 t_:focus-visible:ring-4 t_:focus-visible:outline-hidden t_:disabled:pointer-events-none t_:disabled:opacity-50"
    />
  </SliderRoot>
</template>
