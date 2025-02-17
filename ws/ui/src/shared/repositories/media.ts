import type { ContentNode, PathNodes } from '@panopticum/schemas'
import { invoke } from '@tauri-apps/api/core'
import { defineModule } from '@webshrine/vue'

export const useMediaRepository = defineModule(() => {
  return {
    async getAllMediaLocal(location?: PathNodes): Promise<Extract<ContentNode, { type: 'list' }>> {
      return invoke('content_get_dir_node', {
        location,
      })
    },
  }
})
