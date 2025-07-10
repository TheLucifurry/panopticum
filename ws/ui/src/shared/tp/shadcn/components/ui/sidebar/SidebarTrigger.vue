<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { PanelLeft } from 'lucide-vue-next'
import { Button } from '@/shared/tp/shadcn/components/ui/button'
import { cn } from '@/shared/tp/shadcn/lib/utils'
import { useSidebar } from './utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()

const { toggleSidebar } = useSidebar()
</script>

<template>
  <Button
    data-sidebar="trigger"
    data-slot="sidebar-trigger"
    variant="ghost"
    size="icon"
    :class="cn('t:h-7 t:w-7', props.class)"
    @click="toggleSidebar"
  >
    <PanelLeft />
    <span class="t:sr-only">Toggle Sidebar</span>
  </Button>
</template>
