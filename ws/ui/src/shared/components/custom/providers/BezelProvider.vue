<script setup lang="ts">
import type { VNode } from 'vue'
import type { Nullish } from 'webshrine'
import { shallowRef } from 'vue'
import { Bezel } from '..'

const BEZEL_LIFE_TIME = 1000

let lifetimeTimer: NodeJS.Timeout | undefined
const cont = shallowRef<VNode | string | Nullish>(undefined)

function show(content: VNode | string | Nullish) {
  cont.value = content

  clearTimeout(lifetimeTimer)
  lifetimeTimer = setTimeout(() => {
    if (cont?.value)
      cont.value = undefined
  }, BEZEL_LIFE_TIME)
}

defineExpose({ show })
</script>

<template>
  <div class="z-bezel-provider">
    <Transition>
      <Bezel v-if="cont">
        {{ cont }}
      </Bezel>
    </Transition>
  </div>
</template>

<style scoped>
.z-bezel-provider {
  width: 100%;
  height: 100%;
  position: relative;
  pointer-events: none;
}

.v-enter-active {
  transition: opacity 0.1s ease-out;
}
.v-leave-active {
  transition: opacity 0.3s ease-in;
}
.v-enter-from, .v-leave-to {
  opacity: 0;
}
</style>
