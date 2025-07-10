import type { VariantProps } from 'class-variance-authority'
import type { HTMLAttributes } from 'vue'
import { cva } from 'class-variance-authority'

export interface SidebarProps {
  side?: 'left' | 'right'
  variant?: 'sidebar' | 'floating' | 'inset'
  collapsible?: 'offcanvas' | 'icon' | 'none'
  class?: HTMLAttributes['class']
}

export { default as Sidebar } from './Sidebar.vue'
export { default as SidebarContent } from './SidebarContent.vue'
export { default as SidebarFooter } from './SidebarFooter.vue'
export { default as SidebarGroup } from './SidebarGroup.vue'
export { default as SidebarGroupAction } from './SidebarGroupAction.vue'
export { default as SidebarGroupContent } from './SidebarGroupContent.vue'
export { default as SidebarGroupLabel } from './SidebarGroupLabel.vue'
export { default as SidebarHeader } from './SidebarHeader.vue'
export { default as SidebarInput } from './SidebarInput.vue'
export { default as SidebarInset } from './SidebarInset.vue'
export { default as SidebarMenu } from './SidebarMenu.vue'
export { default as SidebarMenuAction } from './SidebarMenuAction.vue'
export { default as SidebarMenuBadge } from './SidebarMenuBadge.vue'
export { default as SidebarMenuButton } from './SidebarMenuButton.vue'
export { default as SidebarMenuItem } from './SidebarMenuItem.vue'
export { default as SidebarMenuSkeleton } from './SidebarMenuSkeleton.vue'
export { default as SidebarMenuSub } from './SidebarMenuSub.vue'
export { default as SidebarMenuSubButton } from './SidebarMenuSubButton.vue'
export { default as SidebarMenuSubItem } from './SidebarMenuSubItem.vue'
export { default as SidebarProvider } from './SidebarProvider.vue'
export { default as SidebarRail } from './SidebarRail.vue'
export { default as SidebarSeparator } from './SidebarSeparator.vue'
export { default as SidebarTrigger } from './SidebarTrigger.vue'

export { useSidebar } from './utils'

export const sidebarMenuButtonVariants = cva(
  't:peer/menu-button t:flex t:w-full t:items-center t:gap-2 t:overflow-hidden t:rounded-md t:p-2 t:text-left t:text-sm t:outline-hidden t:ring-sidebar-ring t:transition-[width,height,padding] t:hover:bg-sidebar-accent t:hover:text-sidebar-accent-foreground t:focus-visible:ring-2 t:active:bg-sidebar-accent t:active:text-sidebar-accent-foreground t:disabled:pointer-events-none t:disabled:opacity-50 t:group-has-data-[sidebar=menu-action]/menu-item:pr-8 t:aria-disabled:pointer-events-none t:aria-disabled:opacity-50 t:data-[active=true]:bg-sidebar-accent t:data-[active=true]:font-medium t:data-[active=true]:text-sidebar-accent-foreground t:data-[state=open]:hover:bg-sidebar-accent t:data-[state=open]:hover:text-sidebar-accent-foreground t:group-data-[collapsible=icon]:size-8! t:group-data-[collapsible=icon]:p-2! t:[&>span:last-child]:truncate t:[&>svg]:size-4 t:[&>svg]:shrink-0',
  {
    variants: {
      variant: {
        default: 't:hover:bg-sidebar-accent t:hover:text-sidebar-accent-foreground',
        outline:
          't:bg-background t:shadow-[0_0_0_1px_hsl(var(--sidebar-border))] t:hover:bg-sidebar-accent t:hover:text-sidebar-accent-foreground t:hover:shadow-[0_0_0_1px_hsl(var(--sidebar-accent))]',
      },
      size: {
        default: 't:h-8 t:text-sm',
        sm: 't:h-7 t:text-xs',
        lg: 't:h-12 t:text-sm t:group-data-[collapsible=icon]:p-0!',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
)

export type SidebarMenuButtonVariants = VariantProps<typeof sidebarMenuButtonVariants>
