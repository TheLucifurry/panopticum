<script setup lang="ts">
import { useInteraction, useUiState } from '@/shared/modules'
import { SidebarProvider } from '@/shared/tp/shadcn/components/ui/sidebar'
import { Toaster } from '@/shared/tp/shadcn/components/ui/sonner'
import { AppSidebar, WindowBar } from '@/widgets/common'
import { Controls } from '@/widgets/project'

const uis = useUiState()
const i10 = useInteraction()
</script>

<template>
  <div class="desktop">
    <SidebarProvider v-model:open="uis.isSidebarExpanded" class="desktop__layout">
      <AppSidebar />
      <div class="desktop__content">
        <WindowBar>
          <h4 v-if="i10.page.title" class="pl-4 scroll-m-20 text-xl font-semibold tracking-tight">
            {{ i10.page.title }}
          </h4>
          <div id="app__subtitle" />
        </WindowBar>
        <main class="desktop__main">
          <router-view />
        </main>
        <Controls />
      </div>
    </SidebarProvider>

    <Toaster />
  </div>
</template>

<style lang="scss">
.desktop {
  height: 100vh;
  width: 100vw;
  overflow: hidden;

  &__layout {
    width: 100%;
    height: 100%;
    display: grid;
    grid-template: 100% / max-content 1fr;
  }

  &__main {
    overflow: auto;
  }

  &__content {
    display: grid;
    grid-template: max-content 1fr max-content / 100%;
  }
}
</style>
