<script setup lang="ts">
import { Volume, Volume1, Volume2, VolumeOff } from 'lucide-vue-next'
import { Slider } from '@/shared/tp/shadcn/components/ui/slider'
// import Slider from '@/shared/tp/shadcn/components/ui/slider/Slider.vue'

const value = defineModel<number>({
  default: 0,
})
const isMuted = defineModel<boolean>('mute')
</script>

<template>
  <div class="volume">
    <div class="t:cursor-pointer" @click="isMuted = !isMuted">
      <VolumeOff v-if="isMuted" />
      <Volume v-else-if="value === 0" />
      <Volume1 v-else-if="value < 0.5" />
      <Volume2 v-else />
    </div>
    <Slider
      :max="1"
      :step="0.01"
      :model-value="[value]"
      @update:model-value="v => value = v?.[0] || 0"
    />
  </div>
</template>

<style>
.volume {
  display: grid;
  grid-template-columns: max-content 100px;
  gap: 8px;
  padding: 0 4px;
}
</style>
