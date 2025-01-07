<script setup lang="ts">
import { Panel } from '@/shared/components/custom'
import { usePlayer } from '@/shared/modules'
import { WindowBar } from '@/widgets/common'
import { Controls, MediaPlayer } from '@/widgets/project'
import { useElementHover, useIdle } from '@vueuse/core'
import { computed, shallowRef } from 'vue'

const PLAYER_USER_IDLE_TIMEOUT = 3000

const topPanelRef = shallowRef()
const isTopPanelHovered = useElementHover(topPanelRef)
const bottomPanelRef = shallowRef()
const isBottomPanelHovered = useElementHover(bottomPanelRef)

const player = usePlayer()
const idle = useIdle(PLAYER_USER_IDLE_TIMEOUT)
const isInactive = computed(() => idle.idle.value && !isTopPanelHovered.value && !isBottomPanelHovered.value)
</script>

<template>
  <div class="root">
    <MediaPlayer />
    <Panel ref="topPanelRef" :model-value="!isInactive">
      <WindowBar>
        {{ player.currentMedia?.name }}
      </WindowBar>
    </Panel>
    <Panel ref="bottomPanelRef" :model-value="!isInactive" side="bottom">
      <Controls />
    </Panel>
  </div>
</template>

<style lang="scss" scoped>
.root {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>
