<script lang="tsx">
import type { IContentProviderManifest } from '@panopticum/schemas'
import { SidebarGroup, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/shared/components/ui/sidebar'
import { CONTENT_PROVIDER_LIST } from '@/shared/content/contentProviders'
import { ArrowUpIcon, BathIcon, FoldersIcon, PlayCircleIcon, Youtube } from 'lucide-vue-next'
import { defineComponent, h } from 'vue'
import { useRouter } from 'vue-router'

const PROVIDERS = CONTENT_PROVIDER_LIST

const PROVIDERS_ICONS: Record<string, any> = {
  $local_files: FoldersIcon,
  $youtube: Youtube,
  $telegram: ArrowUpIcon,
  $boosty: BathIcon,
}

export default defineComponent({
  setup() {
    const router = useRouter()

    const renderItem = (item: IContentProviderManifest) => {
      return (
        <SidebarMenuItem>
          <SidebarMenuButton
            onClick={() => router.push({ name: 'provider', params: { id: item.id } })}
          >
            {/* @click="router.push({ name: 'provider', params: { id: p.id } })" */}
            {h(PROVIDERS_ICONS[item.id] || PlayCircleIcon, {
              size: 128,
            })}
            <span>{ item.data.name }</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      )

      // <SidebarMenuItem v-for="p in PROVIDERS" :key="p.id">
      //     <CollapsibleTrigger as-child>
      //       <SidebarMenuButton :tooltip="p.data.name">
      //         <component :is="PROVIDERS_ICONS[p.id] || PlayCircleIcon" :size="128" />
      //         <span>{{ p.data.name }}</span>
      //         <ChevronRight class="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
      //       </SidebarMenuButton>
      //     </CollapsibleTrigger>
      //     <CollapsibleContent>
      //       <SidebarMenuSub>
      //         <SidebarMenuSubItem
      //           v-for="subItem in item.items"
      //           :key="subItem.title"
      //         >
      //           <SidebarMenuSubButton as-child>
      //             <a :href="subItem.url">
      //               <span>{{ subItem.title }}</span>
      //             </a>
      //           </SidebarMenuSubButton>
      //         </SidebarMenuSubItem>
      //       </SidebarMenuSub>
      //     </CollapsibleContent>
      //   </SidebarMenuItem>
    }

    return () => (
      <SidebarGroup>
        <SidebarGroupLabel>Providers</SidebarGroupLabel>
        <SidebarMenu>
          {PROVIDERS.map(renderItem)}
        </SidebarMenu>
      </SidebarGroup>
    )
  },
})
</script>

<style lang="scss">
.asd {}
</style>
