<script setup lang="ts">
import { MenuButtonUser } from '@/features/user'
import { SidebarProvider } from '@/shared/components/ui/sidebar'
import { useUiState } from '@/shared/modules'
import { AppSidebar, WindowBar } from '@/widgets/common'
import { Controls, MediaPlayer } from '@/widgets/project'

const uis = useUiState()
</script>

<template>
  <div class="desktop">
    <SidebarProvider v-model:open="uis.isSidebarExpanded" class="desktop__layout">
      <AppSidebar />
      <div class="desktop__content">
        <WindowBar>
          <template #extra>
            <MenuButtonUser />
          </template>
        </WindowBar>
        <main>
          <MediaPlayer />
        </main>
        <Controls />
      </div>
    </SidebarProvider>
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

  &__content {
    display: grid;
    grid-template: max-content 1fr max-content / 100%;
  }
}
</style>
