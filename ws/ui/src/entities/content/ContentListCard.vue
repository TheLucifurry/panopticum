<script setup lang="ts">
import type { IContentList } from '@panopticum/schemas'

const {
  data,
} = defineProps<{
  data: IContentList
}>()
</script>

<template>
  <div class="group relative flex flex-col cursor-pointer" style="border: 1px solid #f0f;">
    <slot />
    {{ data.name }}
  </div>
</template>
