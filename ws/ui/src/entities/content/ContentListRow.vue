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
  <div class="t:relative t:flex t:flex-col t:w-full t:col-span-full">
    <span class="t:text-xl t:font-semibold t:text-gray-900">{{ data.name }}</span>
    <PageGrid>
      <slot />
    </PageGrid>
  </div>
</template>
