<script setup lang="ts">
import { Volume } from '@/entities/common/inputs'
import { MenuButtonSettingsPlayer } from '@/features/player'
import { Group } from '@/shared/components/custom'
import Progress from '@/shared/components/ui/progress/Progress.vue'
import { usePlayer, useUiState } from '@/shared/modules'
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime'
import { Maximize, Minimize, PauseIcon, PlayIcon } from 'lucide-vue-next'

const uis = useUiState()
const player = usePlayer()
</script>

<template>
  <div
    class="controls"
    @focus.capture="(el: FocusEvent) => (el.target as HTMLElement | null)?.blur()"
  >
    <Progress :model-value="player.currentTime" :max="player.trackLengthTime" />
    <div class="panel">
      <Group>
        <!-- <SkipBack /> -->
        <div @click="player.togglePlaying">
          <PlayIcon v-if="!player.isPlaying" />
          <PauseIcon v-else />
        </div>
        <!-- <SkipForward /> -->
        <Volume
          v-model="player.volume"
          v-model:mute="player.isMuted"
        />
        <div class="tw:text-sm">
          <span>{{ toDurationStringFromSeconds(player.currentTime) }}</span>
          {{ ' / ' }}
          <span>{{ toDurationStringFromSeconds(player.trackLengthTime) }}</span>
        </div>
      </Group>
      <div class="panel__spacer" />
      <Group>
        <MenuButtonSettingsPlayer />
        <div @click="uis.toggleFullscreen">
          <Minimize v-if="uis.isFullscreen" />
          <Maximize v-else />
        </div>
      </Group>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.controls {
  padding: 4px;
  display: grid;
  grid-template-rows: max-content 1fr;
}

.panel {
  height: 40px;
  display: flex;
  align-items: center;

  &__spacer {
    flex-grow: 1;
  }
}
</style>
