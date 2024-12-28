<script lang="tsx">
import {UserAvatar} from '@/entities/user';
import DropdownMenu from '@/shared/components/ui/dropdown-menu/DropdownMenu.vue';
import DropdownMenuContent from '@/shared/components/ui/dropdown-menu/DropdownMenuContent.vue';
import DropdownMenuItem from '@/shared/components/ui/dropdown-menu/DropdownMenuItem.vue';
import DropdownMenuLabel from '@/shared/components/ui/dropdown-menu/DropdownMenuLabel.vue';
import DropdownMenuSeparator from '@/shared/components/ui/dropdown-menu/DropdownMenuSeparator.vue';
import DropdownMenuTrigger from '@/shared/components/ui/dropdown-menu/DropdownMenuTrigger.vue';
import { ComponentObjectPropsOptions, defineComponent } from 'vue'

const props: ComponentObjectPropsOptions = {

}

export default defineComponent({
  props,
  setup(props) {

    return () => (
      <DropdownMenu>
        <DropdownMenuTrigger>
          <UserAvatar>L</UserAvatar>
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
  }
})
</script>

