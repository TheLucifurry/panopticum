<script setup lang="ts">
import type { IContentMedia } from '@panopticum/schemas'
import VideoCard from '@/entities/common/VideoCard.vue'
import { usePlayer } from '@/shared/modules'
import { useMediaRepository } from '@/shared/repositories'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useAsync } from '@webshrine/vue/src/composables/useAsync'
import { useRouter } from 'vue-router'

const player = usePlayer()
const router = useRouter()
const mediaRepo = useMediaRepository()

function onMediaClick(media: IContentMedia) {
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
        :title="media.name"
        :thumbnail="media.thumbnailPath ? convertFileSrc(media.thumbnailPath) : ''"
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
