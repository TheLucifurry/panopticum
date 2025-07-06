<script setup lang="ts">
import type { DropdownMenuSubContentEmits, DropdownMenuSubContentProps } from 'reka-ui'
import type { HTMLAttributes } from 'vue'
import { reactiveOmit } from '@vueuse/core'
import {
  DropdownMenuSubContent,

  useForwardPropsEmits,
} from 'reka-ui'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<DropdownMenuSubContentProps & { class?: HTMLAttributes['class'] }>()
const emits = defineEmits<DropdownMenuSubContentEmits>()

const delegatedProps = reactiveOmit(props, 'class')

const forwarded = useForwardPropsEmits(delegatedProps, emits)
</script>

<template>
  <DropdownMenuSubContent
    data-slot="dropdown-menu-sub-content"
    v-bind="forwarded"
    :class="cn('t_:bg-popover t_:text-popover-foreground t_:data-[state=open]:animate-in t_:data-[state=closed]:animate-out t_:data-[state=closed]:fade-out-0 t_:data-[state=open]:fade-in-0 t_:data-[state=closed]:zoom-out-95 t_:data-[state=open]:zoom-in-95 t_:data-[side=bottom]:slide-in-from-top-2 t_:data-[side=left]:slide-in-from-right-2 t_:data-[side=right]:slide-in-from-left-2 t_:data-[side=top]:slide-in-from-bottom-2 t_:z-50 t_:min-w-[8rem] t_:origin-(--reka-dropdown-menu-content-transform-origin) t_:overflow-hidden t_:rounded-md t_:border t_:p-1 t_:shadow-lg', props.class)"
  >
    <slot />
  </DropdownMenuSubContent>
</template>
