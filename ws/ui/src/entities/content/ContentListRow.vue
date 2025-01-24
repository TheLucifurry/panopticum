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
  <div class="group relative flex flex-col w-full cursor-pointer col-span-full" style="border: 1px solid #000;">
    {{ data.name }}
    <PageGrid>
      <slot />
    </PageGrid>
  </div>
</template>
