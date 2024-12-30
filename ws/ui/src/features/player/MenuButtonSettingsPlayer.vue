<script lang="tsx">
import { DropdownMenuSeparator } from '@/shared/components/ui/dropdown-menu';
import DropdownMenu from '@/shared/components/ui/dropdown-menu/DropdownMenu.vue';
import DropdownMenuContent from '@/shared/components/ui/dropdown-menu/DropdownMenuContent.vue';
import DropdownMenuItem from '@/shared/components/ui/dropdown-menu/DropdownMenuItem.vue';
import DropdownMenuSub from '@/shared/components/ui/dropdown-menu/DropdownMenuSub.vue';
import DropdownMenuSubContent from '@/shared/components/ui/dropdown-menu/DropdownMenuSubContent.vue';
import DropdownMenuSubTrigger from '@/shared/components/ui/dropdown-menu/DropdownMenuSubTrigger.vue';
import DropdownMenuTrigger from '@/shared/components/ui/dropdown-menu/DropdownMenuTrigger.vue';
import { Slider } from '@/shared/components/ui/slider';
import { usePlayer } from '@/shared/modules';
import { Settings } from 'lucide-vue-next';
import { DropdownMenuPortal } from 'radix-vue';
import { defineComponent } from 'vue'

const PLAYBACK_SPEED_PRESET = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 1.75, 2]

export default defineComponent({
  setup() {
    const player = usePlayer()

    const handlerChangePlaybackSpeed = (speed: number) => {
      player.playbackSpeed = speed
    }

    const renderItemPlaybackSpeed = () => (
      <DropdownMenuSub>
        <DropdownMenuSubTrigger>
          Playback speed
        </DropdownMenuSubTrigger>
        <DropdownMenuPortal>
          <DropdownMenuSubContent class='w-40'>
            <DropdownMenuItem>
              <div class='w-10'>
                {player.playbackSpeed}
              </div>
              <Slider
                step={0.05}
                modelValue={[player.playbackSpeed]}
                min={0.1}
                max={3}
                onUpdate:modelValue={(value) => value && handlerChangePlaybackSpeed(value[0])}
              />
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            {PLAYBACK_SPEED_PRESET.map((speed)=>(
              <DropdownMenuItem key={speed} onClick={() => handlerChangePlaybackSpeed(speed)}>
                {speed}
              </DropdownMenuItem>
            ))}
          </DropdownMenuSubContent>
        </DropdownMenuPortal>
      </DropdownMenuSub>
    )

    return () => (
      <DropdownMenu>
        <DropdownMenuTrigger>
          <Settings />
        </DropdownMenuTrigger>
        <DropdownMenuContent class='w-56'>
          <DropdownMenuItem>Autoplay</DropdownMenuItem>
          {renderItemPlaybackSpeed()}
        </DropdownMenuContent>
      </DropdownMenu>
    )
  }
})
</script>
