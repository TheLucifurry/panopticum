<script setup lang="ts">
import { useInteraction } from '@/modules/interaction'
import { useUiState } from '@/modules/ui'
import { SidebarProvider } from '@/shared/tp/shadcn/components/ui/sidebar'
import { Toaster } from '@/shared/tp/shadcn/components/ui/sonner'
import { AppSidebar, WindowBar } from '@/shared/components/widgets'
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
          <h4 v-if="i10.page.title" class="t:pl-4 t:scroll-m-20 t:text-xl t:font-semibold t:tracking-tight">
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

<style>
.desktop {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.desktop__layout {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template: 100% / max-content 1fr;
}

.desktop__main {
  overflow: auto;
}

.desktop__content {
  display: grid;
  grid-template: max-content 1fr max-content / 100%;
}
</style>
