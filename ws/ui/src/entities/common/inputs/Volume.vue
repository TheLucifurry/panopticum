<script setup lang="ts">
import Slider from '@/shared/components/ui/slider/Slider.vue'
import { Volume1, Volume2, VolumeOff } from 'lucide-vue-next'

const model = defineModel<number>({
  default: 0,
})
</script>

<template>
  <div class="volume">
    <VolumeOff v-if="model === 0" />
    <Volume1 v-else-if="model < 0.5" />
    <Volume2 v-else />
    <Slider
      :max="1"
      :step="0.05"
      :model-value="[model]"
      @update:model-value="v => model = v?.[0] || 0"
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
