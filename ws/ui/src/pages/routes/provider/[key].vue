<script setup lang="ts">
import PageProviderLocal from '@/pages/components/PageProviderLocal.vue'
import { useInteraction } from '@/shared/modules'
import { computed } from 'vue'
import { useRouter } from 'vue-router'

definePage({ name: 'provider' })

const router = useRouter()
const i10 = useInteraction()

const providerKey = computed(() => router.currentRoute.value.params.key)

i10.page.defineTitle(providerKey)
</script>

<template>
  <div>
    <PageProviderLocal v-if="providerKey === 'local'" />
  </div>
</template>
