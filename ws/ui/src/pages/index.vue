<script setup lang="ts">
import { onUnmounted, watch } from 'vue'
import { useInteraction, usePlayer } from '@/modules'
import { MediaPlayer } from '@/widgets/project'

definePage({ name: 'player' })

const i10 = useInteraction()
const player = usePlayer()

watch(() => player.currentMedia?.name, () => i10.page.title = player.currentMedia?.name || '', { immediate: true })
onUnmounted(() => i10.page.title = '')
</script>

<template>
  <MediaPlayer />
</template>
