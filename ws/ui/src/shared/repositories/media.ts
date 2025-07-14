import type { ContentNode, PathNodes } from '@panopticum/schemas'
import { invoke } from '@tauri-apps/api/core'
import { defineModule } from '@webshrine/vue'
import { useInteraction } from '@/modules'

export const useMediaRepository = defineModule(() => {
  const i10 = useInteraction()

  return {
    async getAllMediaLocal(location?: PathNodes): Promise<Extract<ContentNode, { type: 'list' }>> {
      return invoke('content_get_dir_node', {
        location,
      }).catch((error) => {
        i10.notifyError(error)
        return []
      })
    },
  }
})
