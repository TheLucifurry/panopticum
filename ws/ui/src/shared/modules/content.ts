import { CONTENT_PROVIDER_LIST } from '@/shared/content/contentProviders'
import { defineModule } from '@webshrine/vue'

export const useContent = defineModule(() => {
  const providers = new Map(CONTENT_PROVIDER_LIST.map(p => [p.id, p]))

  return {
    getContentProviderData(id: string) {
      return providers.get(id)?.data || null
    },
    listContentProviders() {
      return CONTENT_PROVIDER_LIST
    },
  }
})
