<script setup lang="ts">
import { TimerIcon } from '@radix-icons/vue'
import { BookmarkIcon, HomeIcon } from 'lucide-vue-next'
import { useRouter } from 'vue-router'
import { MenuButtonUser } from '@/modules/user/components'
import { SidebarProvidersList } from '@/shared/components/entities/sidebar'
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarTrigger } from '@/shared/tp/shadcn/components/ui/sidebar'

const router = useRouter()
</script>

<template>
  <Sidebar collapsible="icon">
    <SidebarHeader>
      <SidebarTrigger class="t:ml-0.5" />
    </SidebarHeader>
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>Application</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton @click="router.push({ name: 'home' })">
                <HomeIcon />
                <span>Home</span>
              </SidebarMenuButton>
            </SidebarMenuItem>
            <SidebarMenuItem>
              <SidebarMenuButton>
                <TimerIcon />
                <span>Watch later</span>
              </SidebarMenuButton>
            </SidebarMenuItem>
            <SidebarMenuItem>
              <SidebarMenuButton asChild>
                <BookmarkIcon />
                <span>Saved</span>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
      <SidebarProvidersList />
    </SidebarContent>
    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem>
          <MenuButtonUser />
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>
  </Sidebar>
</template>
