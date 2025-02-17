<script setup lang="ts">
import type { IContentList } from '@panopticum/schemas'
import { PageGrid } from '@/shared/components/custom'

const {
  data,
} = defineProps<{
  data: IContentList
}>()
</script>

<template>
  <div class="relative flex flex-col w-full col-span-full">
    <span class="text-xl font-semibold text-gray-900">{{ data.name }}</span>
    <PageGrid>
      <slot />
    </PageGrid>
  </div>
</template>
