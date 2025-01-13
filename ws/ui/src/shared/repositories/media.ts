import type { IContentMedia } from '@panopticum/schemas'
import { invoke } from '@tauri-apps/api/core'
import { defineModule } from '@webshrine/vue'

export const useMediaRepository = defineModule(() => {
  return {
    async getAllMediaLocal(): Promise<IContentMedia[]> {
      return invoke<IContentMedia[]>('import_get_all')
    },

  }
})
