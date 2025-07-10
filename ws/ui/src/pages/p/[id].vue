<script setup lang="ts">
import type { PathNodes } from '@panopticum/schemas'
import { PathNode } from '@panopticum/schemas'
import { computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useQueryParam } from '@/shared/composables'
import { useContent, useInteraction } from '@/shared/modules'
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbSeparator } from '@/shared/tp/shadcn/components/ui/breadcrumb'
import { ProviderViewMain } from '@/widgets/provider'

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

watch(providerKey, () => {
  locationPath.value = null
})
</script>

<template>
  <div>
    <teleport defer to="#app__subtitle">
      <Breadcrumb>
        <BreadcrumbList>
          <template v-for="(node, ix) in locationPath" :key="ix">
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbLink>
                {{ PathNode.toLabel(node) }}
              </BreadcrumbLink>
            </BreadcrumbItem>
          </template>
        </BreadcrumbList>
      </Breadcrumb>
    </teleport>
    <ProviderViewMain
      v-if="providerKey === '$local_files'"
      :locationPath="locationPath || undefined"
      @goLocation="onGoLocation"
    />
  </div>
</template>
