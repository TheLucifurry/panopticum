<script setup lang="ts">
import { Volume } from '@/entities/common/inputs';
import { MenuButtonSettingsPlayer } from '@/features/player';
import Progress from '@/shared/components/ui/progress/Progress.vue';
import { usePlayer } from '@/shared/modules';
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime';
import {  Maximize, PlayIcon, SkipForward } from 'lucide-vue-next';

const player = usePlayer()
</script>


<template>
  <div class="controls">
    <Progress :modelValue="player.currentTime" />
    <div class="panel">
      <PlayIcon />
      <SkipForward />
      <Volume :modelValue="player.volume" @update:modelValue="e => player.volume = e" />
      <div class="">
        <span>{{toDurationStringFromSeconds(player.currentTime)}}</span>
        {{' / '}}
        <span>{{toDurationStringFromSeconds(player.trackLengthTime)}}</span>
      </div>
      <div class="panel__spacer"></div>
      <MenuButtonSettingsPlayer />
      <Maximize />
    </div>
  </div>
</template>


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
