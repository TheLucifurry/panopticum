<script setup lang="ts">
import { useInteraction } from '@/modules/interaction'

definePage({ name: 'home' })

const i10 = useInteraction()

i10.page.defineTitle('Home')
</script>

<template>
  <div class="page-home t:p-6" />
</template>

<style>
.page-home {
  width: 100%;
  height: 100%;
}
</style>
