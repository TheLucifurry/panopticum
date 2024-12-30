<script lang="tsx">
import DropdownMenu from '@/shared/components/ui/dropdown-menu/DropdownMenu.vue';
import DropdownMenuContent from '@/shared/components/ui/dropdown-menu/DropdownMenuContent.vue';
import DropdownMenuItem from '@/shared/components/ui/dropdown-menu/DropdownMenuItem.vue';
import DropdownMenuTrigger from '@/shared/components/ui/dropdown-menu/DropdownMenuTrigger.vue';
import { Settings } from 'lucide-vue-next';
import { ComponentObjectPropsOptions, defineComponent } from 'vue'

const props: ComponentObjectPropsOptions = {

}

export default defineComponent({
  props,
  setup(props) {
    return () => (
      <DropdownMenu>
        <DropdownMenuTrigger>
          <Settings />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem>Autoplay</DropdownMenuItem>
          <DropdownMenuItem>Playback speed</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    )
  }
})
</script>

