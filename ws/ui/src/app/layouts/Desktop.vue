<script setup lang="ts">
import { MenuButtonUser } from '@/features/user'
import { Button } from '@/shared/components/ui/button'
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader, SidebarProvider } from '@/shared/components/ui/sidebar'
import { useKeyboard, useUiState } from '@/shared/modules'
import { WindowBar } from '@/widgets/common'
import { Controls, MediaPlayer } from '@/widgets/project'
import { useToggle } from '@vueuse/core'
import { watch } from 'vue'

const uis = useUiState()
const keyboard = useKeyboard()

const [isOpen, toggleSidebar] = useToggle(uis.isSidebarExpanded)

watch(isOpen, v => uis.isSidebarExpanded = v)

keyboard.binds({
  'ctrl > b': () => toggleSidebar(),
})
</script>

<template>
  <div class="root">
    <SidebarProvider :open="isOpen">
      <div class="layout">
        <Sidebar>
          <SidebarHeader>
            Header
          </SidebarHeader>
          <SidebarContent>
            <SidebarGroup />
            <SidebarGroup />
          </SidebarContent>
          <SidebarFooter>
            Footer
          </SidebarFooter>
        </Sidebar>
        <div class="content">
          <WindowBar>
            <Button size="sm" @click="toggleSidebar()">
              Open
            </Button>
            <template #extra>
              <MenuButtonUser />
            </template>
          </WindowBar>
          <main>
            <MediaPlayer />
          </main>
          <Controls />
        </div>
      </div>
    </SidebarProvider>
  </div>
</template>

<style lang="scss" scoped>
.root {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
.layout {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template: 100% / max-content 1fr;
}
.content {
  display: grid;
  grid-template: max-content 1fr max-content / 100%;
}
</style>
