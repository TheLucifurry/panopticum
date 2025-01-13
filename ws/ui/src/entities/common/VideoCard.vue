<script setup lang="ts">
import type { IContentMedia } from '@panopticum/schemas'
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime'
import { useTimeAgo } from '@vueuse/core'

// Define props
const {
  media,
  thumbnail,
  title,
  channelName,
  channelAvatar,
  views,
} = defineProps<{
  media: IContentMedia
  thumbnail: string
  title: string
  channelName?: string
  channelAvatar?: string
  views?: string
}>()

// Define computed or reactive values
const uploaded = useTimeAgo(() => new Date(media.createdAt))
</script>

<template>
  <div class="group relative flex flex-col w-full cursor-pointer">
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

    <div class="mt-3 flex gap-3">
      <img
        v-if="channelAvatar"
        :src="channelAvatar"
        :alt="channelName"
        class="h-10 w-10 rounded-full object-cover"
      >
      <div class="max-w-full">
        <h3 class="w-full text-sm font-medium text-gray-900 line-clamp-2 text-ellipsis" :title="title">
          {{ title }}
        </h3>
        <p class="mt-1 text-sm text-gray-600">
          {{ channelName }}
        </p>
        <p v-if="views && uploaded" class="text-xs text-gray-500">
          {{ views }} • {{ uploaded }}
        </p>
      </div>
    </div>
  </div>
</template>
