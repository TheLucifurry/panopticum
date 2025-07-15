<script setup lang="ts">
import { shallowRef } from 'vue'

interface Props {
  src?: string
  srcFallback?: string
  alt?: string
}

const p = defineProps<Props>()

const currentSrc = shallowRef(p.src)

function handleError() {
  currentSrc.value = p.srcFallback || ''
}
</script>

<template>
  <img
    :src="currentSrc"
    :alt="p.alt"
    @error="handleError"
  >
</template>
