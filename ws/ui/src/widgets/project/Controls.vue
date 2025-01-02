<script setup lang="ts">
import { Volume } from '@/entities/common/inputs';
import { MenuButtonSettingsPlayer } from '@/features/player';
import { Group } from '@/shared/components/custom';
import Progress from '@/shared/components/ui/progress/Progress.vue';
import { usePlayer } from '@/shared/modules';
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime';
import {  Maximize, PlayIcon, PauseIcon, SkipForward } from 'lucide-vue-next';

const player = usePlayer()
</script>

<template>
  <div class="controls">
    <Progress :modelValue="player.currentTime" :max="player.trackLengthTime" />
    <div class="panel">
      <Group>
        <div @click="player.isPlaying = !player.isPlaying">
          <PlayIcon v-if="!player.isPlaying" />
          <PauseIcon v-else />
        </div>
        <SkipForward />
        <Volume :modelValue="player.volume" @update:modelValue="e => player.volume = e" />
        <div class="tw:text-sm">
          <span>{{toDurationStringFromSeconds(player.currentTime)}}</span>
          {{' / '}}
          <span>{{toDurationStringFromSeconds(player.trackLengthTime)}}</span>
        </div>
      </Group>
      <div class="panel__spacer"></div>
      <Group>
        <MenuButtonSettingsPlayer />
        <Maximize />
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
