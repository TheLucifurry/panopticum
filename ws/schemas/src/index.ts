import type { ContentNode, IContentList, IContentMedia } from './gen'

export * from './gen'

export interface ContentNodeWithMedia {
  type: 'media'
  body: IContentMedia
}

export interface ContentNodeWithList {
  type: 'list'
  body: IContentList
}

export const isContentNodeWithMedia = (v: ContentNode): v is ContentNodeWithMedia => v.type === 'media'
export const isContentNodeWithList = (v: ContentNode): v is ContentNodeWithList => v.type === 'list'
