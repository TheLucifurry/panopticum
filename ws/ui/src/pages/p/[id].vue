<script setup lang="ts">
import type { PathNodes } from '@panopticum/schemas'
import { useQueryParam } from '@/shared/composables'
import { useContent, useInteraction } from '@/shared/modules'
import { ProviderViewMain } from '@/widgets/provider'
import { computed, watchEffect } from 'vue'
import { useRoute } from 'vue-router'

definePage({
  name: 'provider',
})

const route = useRoute()
const i10 = useInteraction()
const content = useContent()

const providerKey = computed(() => route.params?.id as string)
const provider = computed(() => content.getContentProviderData(providerKey.value))
const providerName = computed(() => provider.value?.name || '')
const locationPath = useQueryParam<PathNodes>('path', 'sync')

// watchEffect(() => {
//   console.log('locationPath: ', locationPath.value)
// })

i10.page.defineTitle(providerName)

function onGoLocation(newPath: PathNodes) {
  locationPath.value = newPath
}
</script>

<template>
  <div>
    <teleport defer to='#app__subtitle'>
      <span v-for="(n, ix) in locationPath" :key="ix">
        {{ n }}
      </span>
    </teleport>
    <ProviderViewMain
      v-if="providerKey === '$local_files'"
      @go-location="onGoLocation"
    />
  </div>
</template>
