<script setup lang="ts">
import type { IContentMedia } from '@panopticum/schemas'
import { formatTimeAgo } from '@vueuse/core'
import { Img } from '@/shared/components'
import { toDurationStringFromSeconds } from '@/shared/utils/datetime'

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
  <div class="t:group t:relative t:flex t:flex-col t:w-full t:cursor-pointer" @click="$emit('click')">
    <div class="t:relative t:w-full t:aspect-video t:overflow-hidden t:rounded-lg t:bg-gray-300 t:transition-transform t:duration-300 t:group-hover:scale-105">
      <Img
        :src="thumbnail"
        srcFallback="/empty_thumbnail_audio.png"
        :alt="title"
        class="t:h-full t:w-full t:object-cover"
      />
      <span v-if="data.duration" class="t:absolute t:bottom-2 t:right-2 t:px-1.5 t:py-0.5 t:text-xs t:bg-black t:text-white t:rounded">
        {{ toDurationStringFromSeconds(data.duration) }}
      </span>
    </div>

    <div class="t:mt-3 t:flex t:gap-3">
      <img
        v-if="channelAvatar"
        :src="channelAvatar"
        :alt="channelName"
        class="t:h-10 t:w-10 t:rounded-full t:object-cover"
      >
      <div class="t:max-w-full">
        <h3 class="t:w-full t:text-sm t:font-medium t:text-gray-900 t:line-clamp-2 t:text-ellipsis" :title="title">
          {{ title }}
        </h3>
        <p class="t:mt-1 t:text-sm t:text-gray-600">
          {{ channelName }}
        </p>
        <p v-if="views || data.createdAt" class="t:text-xs t:text-gray-500">
          <span v-if="views">{{ views }}</span>
          {{ views && data.createdAt ? '•' : '' }}
          <span v-if="data.createdAt">{{ formatTimeAgo(new Date(data.createdAt)) }}</span>
        </p>
      </div>
    </div>
  </div>
</template>
