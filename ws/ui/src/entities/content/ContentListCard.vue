<script setup lang="ts">
import { Img } from '@/shared/components/custom'
import { type IContentList, isContentNodeWithPreview } from '@panopticum/schemas'
import { computed } from 'vue'
import { maxNumber } from 'webshrine'

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
        v-for="(node, index) in data.items.filter(isContentNodeWithPreview)"
        :key="index"
        class="relative aspect-square overflow-hidden rounded-lg bg-gray-300"
      >
        <Img
          :src="node.body.pict"
          :src-fallback="node.body.pict"
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
