import { invoke } from '@tauri-apps/api/core'
import { defineModule } from '@webshrine/vue'

export interface IMedia {
  name: string
  path: string
  thumbnailPath: string | null
  size: number | null
  mediaType: number
  isLocal: boolean
  duration: number
  createdAt: string
}

export const useMediaRepository = defineModule(() => {
  return {
    async getAllMediaLocal(): Promise<IMedia[]> {
      return invoke<IMedia[]>('import_get_all')
    },
  }
})
