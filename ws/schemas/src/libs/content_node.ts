import type { ContentNode, IContentList, IContentMedia, IContentPreview } from '../gen'

interface ContentNodeWrapper<T, D> { type: T, data: D }

export type ContentNodeWithMedia = ContentNodeWrapper<'media', IContentMedia>
export type ContentNodeWithList = ContentNodeWrapper<'list', IContentList>
export type ContentNodeWithPreview = ContentNodeWrapper<'preview', IContentPreview>

export const isContentNodeWithMedia = (v: ContentNode): v is ContentNodeWithMedia => v.type === 'media'
export const isContentNodeWithList = (v: ContentNode): v is ContentNodeWithList => v.type === 'list'
export const isContentNodeWithPreview = (v: ContentNode): v is ContentNodeWithPreview => v.type === 'preview'
