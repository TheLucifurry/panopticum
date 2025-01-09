<script setup lang="ts">
import type { IMedia } from '@/shared/repositories'
import VideoCard from '@/entities/common/VideoCard.vue'
import { useInteraction, usePlayer } from '@/shared/modules'
import { useMediaRepository } from '@/shared/repositories'
import { useAsync } from '@webshrine/vue/src/composables/useAsync'
import { useRouter } from 'vue-router'

const player = usePlayer()
const router = useRouter()
const i10 = useInteraction()
const mediaRepo = useMediaRepository()

definePage({
  name: 'home',
})

i10.page.defineTitle('Home')

function onMediaClick(media: IMedia) {
  player.setCurrentMedia(media)
  router.push({ name: 'player' })
  player.togglePlaying(true)
}

const medias = useAsync(mediaRepo.getAllMediaLocal, [])
</script>

<template>
  <div class="page-home p-6">
    <!-- Videos -->
    <div class="grid gap-4 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6">
      <VideoCard
        v-for="(media, index) in medias.value"
        :key="index"
        :media="media"
        thumbnail="https://via.placeholder.com/320x180"
        :title="media.name"
        channel-name="Channel Name"
        channel-avatar="https://via.placeholder.com/40"
        views="1.2M views"
        uploaded="2 days ago"
        duration="12:34"
        @click="onMediaClick(media)"
      />
      <!-- Add more VideoCard instances as needed -->
    </div>
  </div>
</template>

<style lang="scss">
.page-home {
  width: 100%;
  height: 100%;
}
</style>
