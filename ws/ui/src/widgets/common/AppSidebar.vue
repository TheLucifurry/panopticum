<script setup lang="ts">
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader } from '@/shared/components/ui/sidebar'
import { useMediaRepository } from '@/shared/repositories'

const mediaRepo = useMediaRepository()
</script>

<template>
  <Sidebar collapsible="icon">
    <SidebarHeader>
      Header
    </SidebarHeader>
    <SidebarContent>
      <div v-for="(media, index) in mediaRepo.allMedia.value" :key="index">
        {{ media.name }}
      </div>
      <SidebarGroup />
      <SidebarGroup />
    </SidebarContent>
    <SidebarFooter>
      Footer
    </SidebarFooter>
  </Sidebar>
</template>

<style lang="scss">
</style>
