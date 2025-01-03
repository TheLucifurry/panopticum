<script setup lang="ts">
import { MenuButtonUser } from '@/features/user'
import { Panel } from '@/shared/components/custom'
import { WindowBar } from '@/widgets/common'
import { Controls, MediaPlayer } from '@/widgets/project'
import { ref } from 'vue'

const asd = ref(true)
</script>

<template>
  <div class="root">
    <MediaPlayer />
    <Panel :model-value="asd">
      <WindowBar>
        <template #extra>
          <MenuButtonUser />
        </template>
      </WindowBar>
    </Panel>
    <Panel side="bottom" :open="asd">
      <Controls />
    </Panel>
  </div>
</template>

<style lang="scss" scoped>
.root {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  color: #fff;
}
</style>
