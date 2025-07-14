<script setup lang="ts">
import { onBeforeUnmount, ref, watch } from 'vue'
import { clampNumber, cope } from 'webshrine'
import { off, on, once } from '@/shared/utils/dom'

interface Props {
  max?: number
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
      class="sensor"
      @mousedown="onSensorMouseDown"
      @mousemove="onSensorMouseMove"
      @mouseleave="onSensorMouseLeave"
    />
    <div class="container">
      <div class="hovered" :style="{ width: getPositionPercent(model, true) }" />
      <div class="remains" :style="{ width: getPositionPercent(hoveredPosition || model, true) }" />
      <div class="thumb" :style="{ left: getPositionPercent(model) }" />
    </div>
  </div>
</template>

<style>
.player-track-bar {
  --width: 4px;
  --sensor-width: calc(var(--width) * 10);
  --thumb-size: 8px;
  --c-main: #111;
  --c-hovered: #fff8;
  --c-remains: #fffc;

  position: relative;
  width: 100%;
  height: var(--width);
  color: var(--c-main);
  background-color: currentColor;

  .container {
    height: 100%;
    width: 100%;
    position: relative;
  }

  .sensor {
    width: 100%;
    height: var(--sensor-width);
    position: absolute;
    bottom: -150%; /* Percent of bottom part */
    cursor: pointer;
    z-index: 1;
  }

  .container>* {
    position: absolute;
    height: 100%;
    pointer-events: none;
  }

  .remains {
    background-color: var(--c-remains);
    right: 0;
  }

  .hovered {
    background-color: var(--c-hovered);
    right: 0;
  }

  .thumb {
    background: currentColor;
    border-radius: 50%;
    width: var(--thumb-size);
    height: var(--thumb-size);
    top: calc(var(--width) / 2);
    transform: translate(-50%, -50%);
  }
}
</style>
