<script setup lang="ts">
import Slider from '@/shared/components/ui/slider/Slider.vue';
import { syncRefs } from '@vueuse/core';
import { VolumeOff, Volume1, Volume2 } from 'lucide-vue-next';
import { shallowRef } from 'vue';

const externalValue = defineModel<number>({
  default: 0
})

const internalValue = shallowRef<number>(externalValue.value)

syncRefs(externalValue, internalValue)
</script>

<template>
  <div class="volume">
    <VolumeOff v-if="internalValue === 0"/>
    <Volume1 v-else-if="internalValue < 0.5"/>
    <Volume2 v-else/>
    <Slider
      :max="1"
      :step="0.05"
      :model-value="[internalValue]"
      @update:model-value="v => internalValue = v?.[0] || 0"
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
