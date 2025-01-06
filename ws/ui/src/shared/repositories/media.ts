import { invoke } from '@tauri-apps/api/core'
import { defineModule } from '@webshrine/vue'
import { useAsync } from '@webshrine/vue/src/composables/useAsync'

interface IMedia {
  name: string
  path: string
  mediaType: number
  isLocal: boolean
}

export const useMediaRepository = defineModule(() => {
  const allMedia = useAsync(() => invoke<IMedia[]>('search_video_files', { searchInput: '' }), [])

  return {
    allMedia,
    async getAll(): Promise<IMedia[]> {
      if (!allMedia.isPending)
        await allMedia.execute()
      return allMedia.value
    },
  }
})
