<script lang="tsx">
import { defineComponent } from 'vue'
import { useUser } from '@/modules/user'
import { UserAvatar } from '@/modules/user/components'
import DropdownMenu from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenu.vue'
import DropdownMenuContent from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuContent.vue'
import DropdownMenuItem from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuItem.vue'
import DropdownMenuLabel from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuLabel.vue'
import DropdownMenuSeparator from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuSeparator.vue'
import DropdownMenuTrigger from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuTrigger.vue'

export default defineComponent({
  setup() {
    const user = useUser()

    return () => (
      <DropdownMenu>
        <DropdownMenuTrigger>
          <UserAvatar>{user.name.initials}</UserAvatar>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuLabel>My Account</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem>Profile</DropdownMenuItem>
          <DropdownMenuItem>Billing</DropdownMenuItem>
          <DropdownMenuItem>Team</DropdownMenuItem>
          <DropdownMenuItem>Subscription</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    )
  },
})
</script>
