<script setup lang="ts">
import type { IMedia } from '@/shared/repositories'
import { Card } from '@/shared/components/ui/card'
import { usePlayer } from '@/shared/modules'
import { useMediaRepository } from '@/shared/repositories'
import { useRouter } from 'vue-router'

const player = usePlayer()
const router = useRouter()
const mediaRepo = useMediaRepository()

definePage({
  name: 'home',
})

function onMediaClick(media: IMedia) {
  player.setCurrentMedia(media)
  router.push({ name: 'player' })
}
</script>

<template>
  <div class="page-home">
    Home page
    <Card
      v-for="(media, index) in mediaRepo.allMedia.value"
      :key="index"
      @click="onMediaClick(media)"
    >
      {{ media.name }}
    </Card>
  </div>
</template>

<style lang="scss">
.page-home {

}
</style>
