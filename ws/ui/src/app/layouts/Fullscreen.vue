<script setup lang="ts">
import { useElementHover, useIdle } from '@vueuse/core'
import { computed, shallowRef } from 'vue'
import { useInteraction } from '@/modules/interaction'
import { Controls, MediaPlayer } from '@/modules/player/components'
import { Panel } from '@/shared/components'
import { WindowBar } from '@/shared/components/widgets'

const PLAYER_USER_IDLE_TIMEOUT = 3000

const topPanelRef = shallowRef()
const isTopPanelHovered = useElementHover(topPanelRef)
const bottomPanelRef = shallowRef()
const isBottomPanelHovered = useElementHover(bottomPanelRef)

const i10 = useInteraction()
const idle = useIdle(PLAYER_USER_IDLE_TIMEOUT)
const isInactive = computed(() => idle.idle.value && !isTopPanelHovered.value && !isBottomPanelHovered.value)
</script>

<template>
  <div class="root">
    <MediaPlayer />
    <Panel ref="topPanelRef" :modelValue="!isInactive">
      <WindowBar>
        <h4 v-if="i10.page.title" class="t:pl-4 t:scroll-m-20 t:text-xl t:font-semibold t:tracking-tight">
          {{ i10.page.title }}
        </h4>
      </WindowBar>
    </Panel>
    <Panel ref="bottomPanelRef" :modelValue="!isInactive" side="bottom">
      <Controls />
    </Panel>
  </div>
</template>

<style scoped>
.root {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>
