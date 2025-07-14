<script setup lang="ts">
import { onUnmounted, watch } from 'vue'
import { useInteraction } from '@/modules/interaction'
import { usePlayer } from '@/modules/player'
import { MediaPlayer } from '@/modules/player/components'

definePage({ name: 'player' })

const i10 = useInteraction()
const player = usePlayer()

watch(() => player.currentMedia?.name, () => i10.page.title = player.currentMedia?.name || '', { immediate: true })
onUnmounted(() => i10.page.title = '')
</script>

<template>
  <MediaPlayer />
</template>
