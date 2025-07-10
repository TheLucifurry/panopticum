<script setup lang="ts">
import type { TooltipContentEmits, TooltipContentProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { TooltipArrow, TooltipContent, TooltipPortal, useForwardPropsEmits } from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

defineOptions({
  inheritAttrs: false,
})

const props = withDefaults(defineProps<TooltipContentProps & { class?: HTMLAttributes['class'] }>(), {
  sideOffset: 4,
})

const emits = defineEmits<TooltipContentEmits>()

const delegatedProps = reactiveOmit(props, 'class')
const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <TooltipPortal>
    <TooltipContent
      data-slot="tooltip-content"
      v-bind="{ ...forwarded, ...$attrs }"
      :class="cn('t:bg-primary t:text-primary-foreground t:animate-in t:fade-in-0 t:zoom-in-95 t:data-[state=closed]:animate-out t:data-[state=closed]:fade-out-0 t:data-[state=closed]:zoom-out-95 t:data-[side=bottom]:slide-in-from-top-2 t:data-[side=left]:slide-in-from-right-2 t:data-[side=right]:slide-in-from-left-2 t:data-[side=top]:slide-in-from-bottom-2 t:z-50 t:w-fit t:rounded-md t:px-3 t:py-1.5 t:text-xs t:text-balance', props.class)"
    >
      <slot />

      <TooltipArrow class="t:bg-primary t:fill-primary t:z-50 t:size-2.5 t:translate-y-[calc(-50%_-_2px)] t:rotate-45 t:rounded-[2px]" />
    </TooltipContent>
  </TooltipPortal>
</template>
