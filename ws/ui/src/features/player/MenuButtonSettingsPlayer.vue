<script lang="tsx">
import { Settings } from 'lucide-vue-next'
import { DropdownMenuPortal } from 'radix-vue'
import { defineComponent } from 'vue'
import { DEFAULT_PLAYBACK_SPEED_OPTIONS, DEFAULT_PLAYBACK_SPEED_RANGE, usePlayer } from '@/modules/player'
import { DropdownMenuSeparator } from '@/shared/tp/shadcn/components/ui/dropdown-menu'
import DropdownMenu from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenu.vue'
import DropdownMenuContent from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuContent.vue'
import DropdownMenuItem from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuItem.vue'
import DropdownMenuSub from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuSub.vue'
import DropdownMenuSubContent from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuSubContent.vue'
import DropdownMenuSubTrigger from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuSubTrigger.vue'
import DropdownMenuTrigger from '@/shared/tp/shadcn/components/ui/dropdown-menu/DropdownMenuTrigger.vue'
import { Slider } from '@/shared/tp/shadcn/components/ui/slider'

export default defineComponent({
  setup() {
    const player = usePlayer()

    const handlerChangePlaybackSpeed = (speed: number) => {
      player.rate = speed
    }

    const renderItemPlaybackSpeed = () => (
      <DropdownMenuSub>
        <DropdownMenuSubTrigger>
          Playback speed
        </DropdownMenuSubTrigger>
        <DropdownMenuPortal>
          <DropdownMenuSubContent class="t:w-40">
            <DropdownMenuItem>
              <div class="t:w-10">
                {player.rate}
              </div>
              <Slider
                step={0.05}
                modelValue={[player.rate]}
                min={DEFAULT_PLAYBACK_SPEED_RANGE[0]}
                max={DEFAULT_PLAYBACK_SPEED_RANGE[1]}
                onUpdate:modelValue={value => value && handlerChangePlaybackSpeed(value[0])}
              />
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            {DEFAULT_PLAYBACK_SPEED_OPTIONS.map(speed => (
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
        <DropdownMenuContent class="t:w-56">
          <DropdownMenuItem>Autoplay</DropdownMenuItem>
          {renderItemPlaybackSpeed()}
        </DropdownMenuContent>
      </DropdownMenu>
    )
  },
})
</script>
