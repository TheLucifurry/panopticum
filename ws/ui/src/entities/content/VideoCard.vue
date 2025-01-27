<script setup lang="ts">
import type { IContentMedia } from '@panopticum/schemas'
import { Img } from '@/shared/components/custom'
import { toDurationStringFromSeconds } from '@/widgets/utils/datetime'
import { formatTimeAgo } from '@vueuse/core'

const {
  data,
  thumbnail,
  title,
  channelName,
  channelAvatar,
  views,
} = defineProps<{
  data: IContentMedia
  thumbnail: string
  title: string
  channelName?: string
  channelAvatar?: string
  views?: string
}>()

defineEmits<{
  click: []
}>()
</script>

<template>
  <div class="group relative flex flex-col w-full cursor-pointer">
    <div class="relative w-full aspect-video overflow-hidden rounded-lg bg-gray-300 transition-transform duration-300 group-hover:scale-105">
      <Img
        :src="thumbnail"
        src-fallback="/empty_thumbnail_audio.png"
        :alt="title"
        class="h-full w-full object-cover"
      />
      <span v-if="data.duration" class="absolute bottom-2 right-2 px-1.5 py-0.5 text-xs bg-black text-white rounded">
        {{ toDurationStringFromSeconds(data.duration) }}
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
        <p v-if="views || data.createdAt" class="text-xs text-gray-500">
          <span v-if="views">{{ views }}</span>
          {{ views && data.createdAt ? '•' : '' }}
          <span v-if="data.createdAt">{{ formatTimeAgo(new Date(data.createdAt)) }}</span>
        </p>
      </div>
    </div>
  </div>
</template>
