<script setup lang="ts">
import { useInteraction } from '@/shared/modules'

definePage({ name: 'home' })

const i10 = useInteraction()

i10.page.defineTitle('Home')
</script>

<template>
  <div class="page-home p-6" />
</template>

<style lang="scss">
.page-home {
  width: 100%;
  height: 100%;
}
</style>
