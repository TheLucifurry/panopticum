<script setup lang="ts">
import { useRouter } from 'vue-router'

definePage({ name: '404' })

const router = useRouter()
</script>

<template>
  <ZPageContent>
    Sorry, the page you visited does not exist.
    <button @click="router.back()">
      Back
    </button>
  </ZPageContent>
</template>
