<script setup lang="ts">
import Slider from '@/shared/components/ui/slider/Slider.vue'
import { Volume, Volume1, Volume2, VolumeOff } from 'lucide-vue-next'

const value = defineModel<number>({
  default: 0,
})
const isMuted = defineModel<boolean>('mute', {
  default: false,
})
</script>

<template>
  <div class="volume">
    <div @click="isMuted = !isMuted">
      <VolumeOff v-if="isMuted" />
      <Volume v-else-if="value === 0" />
      <Volume1 v-else-if="value < 0.5" />
      <Volume2 v-else />
    </div>
    <Slider
      :max="1"
      :step="0.05"
      :model-value="[value]"
      @update:model-value="v => value = v?.[0] || 0"
    />
  </div>
</template>

<style lang="scss">
.volume {
  display: grid;
  grid-template-columns: max-content 64px;
  gap: 8px;
  padding: 0 4px;
}
</style>
