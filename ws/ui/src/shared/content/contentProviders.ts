import type { IContentProviderManifest, IContentProviderV0 } from '@panopticum/schemas'

const createBuildInId = (id: string) => `$${id}`
function createContentProvider(id: string, data: IContentProviderV0): IContentProviderManifest {
  return {
    id: createBuildInId(id),
    version: 0,
    data,
  }
}

const CONTENT_PROVIDER_LOCAL_FILES = createContentProvider('local_files', {
  name: 'Local Files',
  icons: {},
  version: '0',
  views: {
    main: '',
  },
})

const CONTENT_PROVIDER_YOUTUBE = createContentProvider('youtube', {
  name: 'YouTube',
  icons: {},
  version: '0',
  views: {
    main: '',
  },
})

const CONTENT_PROVIDER_SOUND_CLOUD = createContentProvider('sound_cloud', {
  name: 'SoundCloud',
  icons: {},
  version: '0',
  views: {
    main: '',
  },
})

const CONTENT_PROVIDER_TELEGRAM = createContentProvider('telegram', {
  name: 'Telegram',
  icons: {},
  version: '0',
  views: {
    main: '',
  },
})

const CONTENT_PROVIDER_BOOSTY = createContentProvider('boosty', {
  name: 'Boosty',
  icons: {},
  version: '0',
  views: {
    main: '',
  },
})

const CONTENT_PROVIDER_COUB = createContentProvider('coub', {
  name: 'Coub',
  icons: {},
  version: '0',
  views: {
    main: '',
  },
})

export const CONTENT_PROVIDER_LIST: IContentProviderManifest[] = [
  CONTENT_PROVIDER_LOCAL_FILES,
  CONTENT_PROVIDER_YOUTUBE,
  CONTENT_PROVIDER_SOUND_CLOUD,
  CONTENT_PROVIDER_TELEGRAM,
  CONTENT_PROVIDER_BOOSTY,
  CONTENT_PROVIDER_COUB,
]
