<script setup lang="ts">
import type { IContentList } from '@panopticum/schemas'
import { ContentNode } from '@panopticum/schemas'
import { convertFileSrc } from '@tauri-apps/api/core'
import { computed } from 'vue'
import { maxNumber } from 'webshrine'
import { Img } from '@/shared/components'

const {
  data,
} = defineProps<{
  data: IContentList
}>()

const remains = computed(() => maxNumber(data.page.total - 3, 0))
</script>

<template>
  <div class="t:bg-white t:rounded-lg">
    <div class="t:grid t:grid-cols-2 t:gap-2">
      <div
        v-for="(node, index) in data.items.filter(ContentNode.isWithPreview)"
        :key="index"
        class="t:relative t:aspect-square t:overflow-hidden t:rounded-lg t:bg-gray-300"
      >
        <Img
          :src="node.data.pict ? convertFileSrc(node.data.pict) : ''"
          srcFallback="/empty_thumbnail_audio.png"
          class="t:h-full t:w-full t:object-cover t:transition-transform t:duration-300 t:hover:scale-105"
        />
      </div>

      <div
        v-if="remains"
        class="t:relative t:flex t:items-center t:justify-center t:rounded-lg t:bg-gray-200"
      >
        <span class="t:text-lg t:font-medium t:text-gray-600">+{{ remains }}</span>
      </div>
    </div>
    <h3 class="t:mb-3 t:text-sm t:font-medium t:text-gray-900 t:text-ellipsis">
      {{ data.name }}
    </h3>
  </div>
</template>
