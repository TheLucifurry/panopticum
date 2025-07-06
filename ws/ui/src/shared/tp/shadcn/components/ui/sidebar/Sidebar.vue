<script setup lang="ts">
import type { SidebarProps } from '.'
import { Sheet, SheetContent } from '@/shared/tp/shadcn/components/ui/sheet'
import SheetDescription from '@/shared/tp/shadcn/components/ui/sheet/SheetDescription.vue'
import SheetHeader from '@/shared/tp/shadcn/components/ui/sheet/SheetHeader.vue'
import SheetTitle from '@/shared/tp/shadcn/components/ui/sheet/SheetTitle.vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'
import { SIDEBAR_WIDTH_MOBILE, useSidebar } from './utils'

defineOptions({
  inheritAttrs: false,
})

const props = withDefaults(defineProps<SidebarProps>(), {
  side: 'left',
  variant: 'sidebar',
  collapsible: 'offcanvas',
})

const { isMobile, state, openMobile, setOpenMobile } = useSidebar()
</script>

<template>
  <div
    v-if="collapsible === 'none'"
    data-slot="sidebar"
    :class="cn('t_:bg-sidebar t_:text-sidebar-foreground t_:flex t_:h-full t_:w-(--sidebar-width) t_:flex-col', props.class)"
    v-bind="$attrs"
  >
    <slot />
  </div>

  <Sheet v-else-if="isMobile" :open="openMobile" v-bind="$attrs" @update:open="setOpenMobile">
    <SheetContent
      data-sidebar="sidebar"
      data-slot="sidebar"
      data-mobile="true"
      :side="side"
      class="t_:bg-sidebar t_:text-sidebar-foreground t_:w-(--sidebar-width) t_:p-0 t_:[&>button]:hidden"
      :style="{
        '--sidebar-width': SIDEBAR_WIDTH_MOBILE,
      }"
    >
      <SheetHeader class="t_:sr-only">
        <SheetTitle>Sidebar</SheetTitle>
        <SheetDescription>Displays the mobile sidebar.</SheetDescription>
      </SheetHeader>
      <div class="t_:flex t_:h-full t_:w-full t_:flex-col">
        <slot />
      </div>
    </SheetContent>
  </Sheet>

  <div
    v-else
    class="t_:group t_:peer t_:text-sidebar-foreground t_:hidden t_:md:block"
    data-slot="sidebar"
    :data-state="state"
    :data-collapsible="state === 'collapsed' ? collapsible : ''"
    :data-variant="variant"
    :data-side="side"
  >
    <!-- This is what handles the sidebar gap on desktop  -->
    <div
      :class="cn(
        't_:relative t_:w-(--sidebar-width) t_:bg-transparent t_:transition-[width] t_:duration-200 t_:ease-linear',
        't_:group-data-[collapsible=offcanvas]:w-0',
        't_:group-data-[side=right]:rotate-180',
        variant === 'floating' || variant === 'inset'
          ? 't_:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]'
          : 't_:group-data-[collapsible=icon]:w-(--sidebar-width-icon)',
      )"
    />
    <div
      :class="cn(
        't_:fixed t_:inset-y-0 t_:z-10 t_:hidden t_:h-svh t_:w-(--sidebar-width) t_:transition-[left,right,width] t_:duration-200 t_:ease-linear t_:md:flex',
        side === 'left'
          ? 't_:left-0 t_:group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]'
          : 't_:right-0 t_:group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]',
        // Adjust the padding for floating and inset variants.
        variant === 'floating' || variant === 'inset'
          ? 't_:p-2 t_:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]'
          : 't_:group-data-[collapsible=icon]:w-(--sidebar-width-icon) t_:group-data-[side=left]:border-r t_:group-data-[side=right]:border-l',
        props.class,
      )"
      v-bind="$attrs"
    >
      <div
        data-sidebar="sidebar"
        class="t_:bg-sidebar t_:group-data-[variant=floating]:border-sidebar-border t_:flex t_:h-full t_:w-full t_:flex-col t_:group-data-[variant=floating]:rounded-lg t_:group-data-[variant=floating]:border t_:group-data-[variant=floating]:shadow-sm"
      >
        <slot />
      </div>
    </div>
  </div>
</template>
