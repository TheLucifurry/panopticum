<script lang="ts">
import type { IMedia } from '@/shared/repositories'
import type { PropType } from 'vue'
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime'
import { useTimeAgo } from '@vueuse/core'
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'VideoCard',
  props: {
    media: {
      type: Object as PropType<IMedia>,
      required: true,
    },
    thumbnail: {
      type: String,
      required: true,
    },
    title: {
      type: String,
      required: true,
    },
    channelName: {
      type: String,
      required: true,
    },
    channelAvatar: {
      type: String,
      required: true,
    },
    views: {
      type: String,
      required: true,
    },
  },
  setup(props) {
    return {
      toDurationStringFromSeconds,
      uploaded: useTimeAgo(() => new Date(props.media.createdAt)),
    }
  },
})
</script>

<template>
  <div class="group relative flex flex-col w-full cursor-pointer">
    <!-- Thumbnail -->
    <div class="relative w-full aspect-video overflow-hidden rounded-lg bg-gray-300 transition-transform duration-300 group-hover:scale-105">
      <img
        :src="thumbnail"
        :alt="title"
        class="h-full w-full object-cover"
      >
      <span v-if="media.duration" class="absolute bottom-2 right-2 px-1.5 py-0.5 text-xs bg-black text-white rounded">
        {{ toDurationStringFromSeconds(media.duration) }}
      </span>
    </div>

    <!-- Video Details -->
    <div class="mt-3 flex">
      <!-- Avatar -->
      <img
        :src="channelAvatar"
        :alt="channelName"
        class="h-10 w-10 rounded-full object-cover"
      >
      <!-- Info -->
      <div class="ml-3">
        <h3 class="text-sm font-medium text-gray-900 line-clamp-2">
          {{ title }}
        </h3>
        <p class="mt-1 text-sm text-gray-600">
          {{ channelName }}
        </p>
        <p class="text-xs text-gray-500">
          {{ views }} • {{ uploaded }}
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Custom styles if needed */
</style>
