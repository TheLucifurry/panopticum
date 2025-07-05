<script setup lang="ts">
import type { IContentList } from '@panopticum/schemas'
import { ContentNode } from '@panopticum/schemas'
import { convertFileSrc } from '@tauri-apps/api/core'
import { computed } from 'vue'
import { maxNumber } from 'webshrine'
import { Img } from '@/shared/components/custom'

const {
  data,
} = defineProps<{
  data: IContentList
}>()

const remains = computed(() => maxNumber(data.page.total - 3, 0))
</script>

<template>
  <div class="bg-white rounded-lg">
    <div class="grid grid-cols-2 gap-2">
      <div
        v-for="(node, index) in data.items.filter(ContentNode.isWithPreview)"
        :key="index"
        class="relative aspect-square overflow-hidden rounded-lg bg-gray-300"
      >
        <Img
          :src="node.data.pict ? convertFileSrc(node.data.pict) : ''"
          src-fallback="/empty_thumbnail_audio.png"
          class="h-full w-full object-cover transition-transform duration-300 hover:scale-105"
        />
      </div>

      <div
        v-if="remains"
        class="relative flex items-center justify-center rounded-lg bg-gray-200"
      >
        <span class="text-lg font-medium text-gray-600">+{{ remains }}</span>
      </div>
    </div>
    <h3 class="mb-3 text-sm font-medium text-gray-900 text-ellipsis">
      {{ data.name }}
    </h3>
  </div>
</template>
