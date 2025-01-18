<script setup lang="ts">
import { useInteraction, usePlayer } from '@/shared/modules'
import { MediaPlayer } from '@/widgets/project'
import { onUnmounted, watch } from 'vue'

definePage({ name: 'player' })

const i10 = useInteraction()
const player = usePlayer()

watch(() => player.currentMedia?.name, () => i10.page.title = player.currentMedia?.name || '', { immediate: true })
onUnmounted(() => i10.page.title = '')
</script>

<template>
  <MediaPlayer />
</template>
