<script setup lang="ts">
type Side = 'top' | 'bottom' | 'left' | 'right'

const {
  side = 'top',
} = defineProps<{
  side?: Side
}>()

const model = defineModel<boolean>({ default: true })
</script>

<template>
  <div
    class="z-panel"
    :class="{
      'z-panel--open': model,
      'z-panel--vertical': side === 'left' || side === 'right',
    }"
    :style="{ [side]: 0 }"
  >
    <slot />
  </div>
</template>

<style>
.z-panel {
  position: absolute;
  background-color: #0004;

  height: max-content;
  width: 100%;
  transition: opacity .3s ease-out;

  &--vertical {
    height: 100%;
    width: max-content;
  }

  &:not(&--open) {
    opacity: 0;
  }
}
</style>
