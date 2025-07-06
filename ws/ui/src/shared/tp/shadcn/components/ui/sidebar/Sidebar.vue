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
    :class="cn('t:bg-sidebar t:text-sidebar-foreground t:flex t:h-full t:w-(--sidebar-width) t:flex-col', props.class)"
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
      class="t:bg-sidebar t:text-sidebar-foreground t:w-(--sidebar-width) t:p-0 t:[&>button]:hidden"
      :style="{
        '--sidebar-width': SIDEBAR_WIDTH_MOBILE,
      }"
    >
      <SheetHeader class="t:sr-only">
        <SheetTitle>Sidebar</SheetTitle>
        <SheetDescription>Displays the mobile sidebar.</SheetDescription>
      </SheetHeader>
      <div class="t:flex t:h-full t:w-full t:flex-col">
        <slot />
      </div>
    </SheetContent>
  </Sheet>

  <div
    v-else
    class="t:group t:peer t:text-sidebar-foreground t:hidden t:md:block"
    data-slot="sidebar"
    :data-state="state"
    :data-collapsible="state === 'collapsed' ? collapsible : ''"
    :data-variant="variant"
    :data-side="side"
  >
    <!-- This is what handles the sidebar gap on desktop  -->
    <div
      :class="cn(
        't:relative t:w-(--sidebar-width) t:bg-transparent t:transition-[width] t:duration-200 t:ease-linear',
        't:group-data-[collapsible=offcanvas]:w-0',
        't:group-data-[side=right]:rotate-180',
        variant === 'floating' || variant === 'inset'
          ? 't:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]'
          : 't:group-data-[collapsible=icon]:w-(--sidebar-width-icon)',
      )"
    />
    <div
      :class="cn(
        't:fixed t:inset-y-0 t:z-10 t:hidden t:h-svh t:w-(--sidebar-width) t:transition-[left,right,width] t:duration-200 t:ease-linear t:md:flex',
        side === 'left'
          ? 't:left-0 t:group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]'
          : 't:right-0 t:group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]',
        // Adjust the padding for floating and inset variants.
        variant === 'floating' || variant === 'inset'
          ? 't:p-2 t:group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]'
          : 't:group-data-[collapsible=icon]:w-(--sidebar-width-icon) t:group-data-[side=left]:border-r t:group-data-[side=right]:border-l',
        props.class,
      )"
      v-bind="$attrs"
    >
      <div
        data-sidebar="sidebar"
        class="t:bg-sidebar t:group-data-[variant=floating]:border-sidebar-border t:flex t:h-full t:w-full t:flex-col t:group-data-[variant=floating]:rounded-lg t:group-data-[variant=floating]:border t:group-data-[variant=floating]:shadow-sm"
      >
        <slot />
      </div>
    </div>
  </div>
</template>
