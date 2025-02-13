import type { ContentNode, IContentList, IContentMedia, IContentPreview } from './gen'

export * from './gen'
export * from './libs'

export interface ContentNodeWithMedia {
  type: 'media'
  body: IContentMedia
}

export interface ContentNodeWithList {
  type: 'list'
  body: IContentList
}

export interface ContentNodeWithPreview {
  type: 'preview'
  body: IContentPreview
}

export const isContentNodeWithMedia = (v: ContentNode): v is ContentNodeWithMedia => v.type === 'media'
export const isContentNodeWithList = (v: ContentNode): v is ContentNodeWithList => v.type === 'list'
export const isContentNodeWithPreview = (v: ContentNode): v is ContentNodeWithPreview => v.type === 'preview'
