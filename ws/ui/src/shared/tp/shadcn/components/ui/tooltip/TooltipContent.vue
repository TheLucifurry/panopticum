<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import { TooltipArrow, TooltipContent, type TooltipContentEmits, type TooltipContentProps, TooltipPortal, useForwardPropsEmits } from 'reka-ui'
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
      :class="cn('t_:bg-primary t_:text-primary-foreground t_:animate-in t_:fade-in-0 t_:zoom-in-95 t_:data-[state=closed]:animate-out t_:data-[state=closed]:fade-out-0 t_:data-[state=closed]:zoom-out-95 t_:data-[side=bottom]:slide-in-from-top-2 t_:data-[side=left]:slide-in-from-right-2 t_:data-[side=right]:slide-in-from-left-2 t_:data-[side=top]:slide-in-from-bottom-2 t_:z-50 t_:w-fit t_:rounded-md t_:px-3 t_:py-1.5 t_:text-xs t_:text-balance', props.class)"
    >
      <slot />

      <TooltipArrow class="t_:bg-primary t_:fill-primary t_:z-50 t_:size-2.5 t_:translate-y-[calc(-50%_-_2px)] t_:rotate-45 t_:rounded-[2px]" />
    </TooltipContent>
  </TooltipPortal>
</template>
