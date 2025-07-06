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
  't_:peer/menu-button t_:flex t_:w-full t_:items-center t_:gap-2 t_:overflow-hidden t_:rounded-md t_:p-2 t_:text-left t_:text-sm t_:outline-hidden t_:ring-sidebar-ring t_:transition-[width,height,padding] t_:hover:bg-sidebar-accent t_:hover:text-sidebar-accent-foreground t_:focus-visible:ring-2 t_:active:bg-sidebar-accent t_:active:text-sidebar-accent-foreground t_:disabled:pointer-events-none t_:disabled:opacity-50 t_:group-has-data-[sidebar=menu-action]/menu-item:pr-8 t_:aria-disabled:pointer-events-none t_:aria-disabled:opacity-50 t_:data-[active=true]:bg-sidebar-accent t_:data-[active=true]:font-medium t_:data-[active=true]:text-sidebar-accent-foreground t_:data-[state=open]:hover:bg-sidebar-accent t_:data-[state=open]:hover:text-sidebar-accent-foreground t_:group-data-[collapsible=icon]:size-8! t_:group-data-[collapsible=icon]:p-2! t_:[&>span:last-child]:truncate t_:[&>svg]:size-4 t_:[&>svg]:shrink-0',
  {
    variants: {
      variant: {
        default: 't_:hover:bg-sidebar-accent t_:hover:text-sidebar-accent-foreground',
        outline:
          't_:bg-background t_:shadow-[0_0_0_1px_hsl(var(--sidebar-border))] t_:hover:bg-sidebar-accent t_:hover:text-sidebar-accent-foreground t_:hover:shadow-[0_0_0_1px_hsl(var(--sidebar-accent))]',
      },
      size: {
        default: 't_:h-8 t_:text-sm',
        sm: 't_:h-7 t_:text-xs',
        lg: 't_:h-12 t_:text-sm t_:group-data-[collapsible=icon]:p-0!',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
)

export type SidebarMenuButtonVariants = VariantProps<typeof sidebarMenuButtonVariants>
