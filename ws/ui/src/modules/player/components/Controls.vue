<script setup lang="ts">
import { Maximize, Minimize, PauseIcon, PlayIcon } from 'lucide-vue-next'
import { usePlayer } from '@/modules/player'
import { MenuButtonSettingsPlayer } from '@/modules/player/components'
import { useUiState } from '@/modules/ui'
import { Group } from '@/shared/components'
import { Volume } from '@/shared/components/entities/inputs'
import { toDurationStringFromSeconds } from '@/shared/utils/datetime'
import ControlsTrack from './ControlsTrack.vue'

const uis = useUiState()
const player = usePlayer()
</script>

<template>
  <div
    class="controls"
    @focus.capture="(el: FocusEvent) => (el.target as HTMLElement | null)?.blur()"
  >
    <ControlsTrack v-model="player.currentTime" :max="player.duration" />
    <div class="panel">
      <Group>
        <!-- <SkipBack /> -->
        <div class="cursor-pointer" @click="player.togglePlaying()">
          <PlayIcon v-if="!player.isPlaying" />
          <PauseIcon v-else />
        </div>
        <!-- <SkipForward /> -->
        <Volume
          v-model="player.volume"
          v-model:mute="player.isMuted"
        />
        <div class="t:text-sm">
          <span>{{ toDurationStringFromSeconds(player.currentTime) }}</span>
          {{ ' / ' }}
          <span>{{ toDurationStringFromSeconds(player.duration) }}</span>
        </div>
      </Group>
      <div class="spacer" />
      <Group>
        <MenuButtonSettingsPlayer />
        <div class="cursor-pointer" @click="uis.toggleFullscreen">
          <Minimize v-if="uis.isFullscreen" />
          <Maximize v-else />
        </div>
      </Group>
    </div>
  </div>
</template>

<style scoped>
.controls {
  padding: 4px;
  display: grid;
  grid-template-rows: max-content 1fr;
}

.panel {
  height: 40px;
  display: flex;
  align-items: center;

  .spacer {
    flex-grow: 1;
  }
}
</style>
