<script setup lang="ts">
import { off, on, once } from '@/shared/utils/dom'
import { onBeforeUnmount, ref, watch } from 'vue'
import { clampNumber, cope } from 'webshrine'

interface Props {
  max: number | undefined
}

const props = withDefaults(defineProps<Props>(), {
  max: 1,
})

const model = defineModel({ default: 0 })

const internalValue = ref(model)
const sensorEl = ref<HTMLElement | null>(null)
const hoveredPosition = ref(0)

function getPositionPercent(value: number, inverted = false): string {
  const percent = value / props.max
  return `${(inverted ? 1 - percent : percent) * 100}%`
}
function getPositionFromMouseEvent(event: MouseEvent): number {
  const [value = 0] = cope(() => {
    const el = sensorEl.value!
    const positionPrecent = (event.clientX - el.getBoundingClientRect().left) / el.offsetWidth
    return clampNumber(positionPrecent * props.max, 0, props.max)
  })

  return value
}

function onGlobalMouseMove(event: MouseEvent) {
  internalValue.value = getPositionFromMouseEvent(event)
}
function onGlobalMouseUp() {
  // document.body.removeEventListener('mousemove', onGlobalMouseMove)
  off(document.body, 'mousemove', onGlobalMouseMove)
}
function onSensorMouseDown(event: MouseEvent) {
  internalValue.value = getPositionFromMouseEvent(event)
  on(document.body, 'mousemove', onGlobalMouseMove)
  once(document.body, 'mouseup', onGlobalMouseUp)
  event.preventDefault()
}
function onSensorMouseMove(event: MouseEvent) {
  const pos = getPositionFromMouseEvent(event)
  hoveredPosition.value = Math.max(pos, model.value)
}
function onSensorMouseLeave() {
  hoveredPosition.value = 0
}

watch(internalValue, (newValue) => {
  model.value = newValue
})

onBeforeUnmount(() => {
  off(document.body, 'mousemove', onGlobalMouseMove)
  off(document.body, 'mouseup', onGlobalMouseUp)
})
</script>

<template>
  <div class="player-track-bar">
    <div
      ref="sensorEl"
      class="player-track-bar__sensor"
      @mousedown="onSensorMouseDown"
      @mousemove="onSensorMouseMove"
      @mouseleave="onSensorMouseLeave"
    />
    <div class="player-track-bar__container">
      <div class="player-track-bar__hovered" :style="{ width: getPositionPercent(model, true) }" />
      <div class="player-track-bar__remains" :style="{ width: getPositionPercent(hoveredPosition || model, true) }" />
      <div class="player-track-bar__thumb" :style="{ left: getPositionPercent(model) }" />
    </div>
  </div>
</template>

<style lang="scss">
$player-track-bar__width: 4px;
$player-track-bar__sensor-width: calc($player-track-bar__width * 10);
$player-track-bar__thumb-size: 8px;

$color-main: #111;
$color-hovered: #fff8;
$color-remains: #fffc;

.player-track-bar {
  position: relative;
  width: 100%;
  height: $player-track-bar__width;
  color: $color-main;
  background-color: currentColor;

  &__container {
    height: 100%;
    width: 100%;
    position: relative;
  }

  &__sensor {
    width: 100%;
    height: $player-track-bar__sensor-width;
    position: absolute;
    bottom: -150%; // Percent of bottom part
    cursor: pointer;
    z-index: 1;
  }

  &__container>* {
    position: absolute;
    height: 100%;
    pointer-events: none;
  }

  &__remains {
    background-color: $color-remains;
    right: 0;
  }

  &__hovered {
    background-color: $color-hovered;
    right: 0;
  }

  &__thumb {
    background: currentColor;
    border-radius: 50%;
    width: $player-track-bar__thumb-size;
    height: $player-track-bar__thumb-size;
    top: calc($player-track-bar__width / 2);
    transform: translate(-50%, -50%);
  }
}
</style>
