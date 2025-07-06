<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  DropdownMenuContent,
  type DropdownMenuContentEmits,
  type DropdownMenuContentProps,
  DropdownMenuPortal,
  useForwardPropsEmits,
} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = withDefaults(
  defineProps<DropdownMenuContentProps & { class?: HTMLAttributes['class'] }>(),
  {
    sideOffset: 4,
  },
)
const emits = defineEmits<DropdownMenuContentEmits>()

const delegatedProps = reactiveOmit(props, 'class')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DropdownMenuPortal>
    <DropdownMenuContent
      data-slot="dropdown-menu-content"
      v-bind="forwarded"
      :class="cn('t_:bg-popover t_:text-popover-foreground t_:data-[state=open]:animate-in t_:data-[state=closed]:animate-out t_:data-[state=closed]:fade-out-0 t_:data-[state=open]:fade-in-0 t_:data-[state=closed]:zoom-out-95 t_:data-[state=open]:zoom-in-95 t_:data-[side=bottom]:slide-in-from-top-2 t_:data-[side=left]:slide-in-from-right-2 t_:data-[side=right]:slide-in-from-left-2 t_:data-[side=top]:slide-in-from-bottom-2 t_:z-50 t_:max-h-(--reka-dropdown-menu-content-available-height) t_:min-w-[8rem] t_:origin-(--reka-dropdown-menu-content-transform-origin) t_:overflow-x-hidden t_:overflow-y-auto t_:rounded-md t_:border t_:p-1 t_:shadow-md', props.class)"
    >
      <slot />
    </DropdownMenuContent>
  </DropdownMenuPortal>
</template>
