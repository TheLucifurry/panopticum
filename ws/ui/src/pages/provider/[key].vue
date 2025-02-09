<script setup lang="ts">
import { useContent, useInteraction } from '@/shared/modules'
import { ProviderViewMain } from '@/widgets/provider'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

definePage({
  name: 'provider',
})

const route = useRoute()
const i10 = useInteraction()
const content = useContent()

const providerKey = computed(() => route.params?.key as string)
const provider = computed(() => content.getContentProviderData(providerKey.value))
const providerName = computed(() => provider.value?.name || '')

i10.page.defineTitle(providerName)
</script>

<template>
  <div>
    <ProviderViewMain v-if="providerKey === '$local_files'" />
  </div>
</template>
