import type { ContentNode } from '@panopticum/schemas'
import { invoke } from '@tauri-apps/api/core'
import { defineModule } from '@webshrine/vue'

export const useMediaRepository = defineModule(() => {
  return {
    async getAllMediaLocal(): Promise<ContentNode[]> {
      return invoke<ContentNode[]>('content_get_all')
    },
  }
})
