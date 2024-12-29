<script lang="tsx">
import { Volume } from '@/entities/common/inputs';
import Progress from '@/shared/components/ui/progress/Progress.vue';
import { usePlayer } from '@/shared/modules';
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime';
import {  Maximize, PlayIcon, Settings, SkipForward } from 'lucide-vue-next';
import { ComponentObjectPropsOptions, defineComponent } from 'vue'

const props: ComponentObjectPropsOptions = {

}

export default defineComponent({
  props,
  setup(props) {
    const player = usePlayer()

    return () => (
      <div class="controls">
        <Progress modelValue={player.currentTime}/>
        <div class="panel">
          <PlayIcon />
          <SkipForward />
          <Volume modelValue={player.volume} onUpdate:modelValue={e => player.volume = e} />
          <div class="">
            <span>{toDurationStringFromSeconds(player.currentTime)}</span>
            {' / '}
            <span>{toDurationStringFromSeconds(player.trackLengthTime)}</span>
          </div>
          <div class="panel__spacer"></div>
          <Settings />
          <Maximize />
        </div>
      </div>
    )
  }
})
</script>


<style lang="scss" scoped>
.controls {
  height: 64px;
  background-color: #0002;
  padding: 4px;
  display: grid;
  grid-template-rows: max-content 1fr;
}

.panel {
  display: flex;
  align-items: center;

  &__spacer {
    flex-grow: 1;
  }
}
</style>
