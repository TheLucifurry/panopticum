<script setup lang="ts">
import type { IContentProvider } from '@panopticum/schemas'
import { MenuButtonUser } from '@/features/user'
import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarTrigger } from '@/shared/components/ui/sidebar'
import { TimerIcon } from '@radix-icons/vue'
import { ArrowUpIcon, BathIcon, BookmarkIcon, FoldersIcon, HomeIcon, Youtube } from 'lucide-vue-next'
import { useRouter } from 'vue-router'

const PROVIDERS: IContentProvider[] = [
  {
    key: 'local',
    name: 'Local files',
  },
  {
    key: 'youtube',
    name: 'YouTube',
  },
  {
    key: 'telegram',
    name: 'Telegram',
  },
  {
    key: 'boosty',
    name: 'Boosty',
  },
]

const PROVIDERS_ICONS: Record<IContentProvider['key'], any> = {
  local: FoldersIcon,
  youtube: Youtube,
  telegram: ArrowUpIcon,
  boosty: BathIcon,
}

const router = useRouter()
</script>

<template>
  <Sidebar collapsible="icon">
    <SidebarHeader>
      <SidebarTrigger class="ml-0.5" />
    </SidebarHeader>
    <SidebarContent>

      <SidebarGroup>
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
            <SidebarMenuButton>
              <BookmarkIcon :size="64" />
              <span>Saved</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>

      <SidebarGroup>
        <SidebarGroupLabel>Providers</SidebarGroupLabel>
        <SidebarMenu>
          <SidebarMenuItem v-for="p in PROVIDERS" :key="p.key">
            <SidebarMenuButton @click="router.push({ name: 'provider', params: { key: p.key } })">
              <component :is="PROVIDERS_ICONS[p.key]" :size="128" />
              <span>{{ p.name }}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroup>

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
