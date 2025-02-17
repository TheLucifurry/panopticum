import type { IContentList, IContentMedia, IContentPreview } from '../gen'

export type { ContentNode } from '../gen'

interface ContentNodeWrapper<T, D> { type: T, data: D }

export type ContentNodeWithMedia = ContentNodeWrapper<'media', IContentMedia>
export type ContentNodeWithList = ContentNodeWrapper<'list', IContentList>
export type ContentNodeWithPreview = ContentNodeWrapper<'preview', IContentPreview>

export namespace ContentNode {
  export const isWithMedia = (v: ContentNode): v is ContentNodeWithMedia => v.type === 'media'
  export const isWithList = (v: ContentNode): v is ContentNodeWithList => v.type === 'list'
  export const isWithPreview = (v: ContentNode): v is ContentNodeWithPreview => v.type === 'preview'
}
