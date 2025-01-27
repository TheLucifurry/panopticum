import type { ContentNode } from '@panopticum/schemas'
import type { Component, VNode } from 'vue'
import { isContentNodeWithList, isContentNodeWithMedia } from '@panopticum/schemas'
import { convertFileSrc } from '@tauri-apps/api/core'
import { h } from 'vue'
import { noop } from 'webshrine'
import ContentListCard from './ContentListCard.vue'
import ContentListRow from './ContentListRow.vue'
import VideoCard from './VideoCard.vue'

interface IRenderContentNodeCardProps {
  rendererChildren?: Component
  onCardClick?: (node: ContentNode) => void
}

export function renderContentNodeCard(node: ContentNode, properties: IRenderContentNodeCardProps = {}): VNode {
  const props: Required<IRenderContentNodeCardProps> = {
    rendererChildren: ContentListRow,
    onCardClick: noop,
    ...properties,
  }

  const handleClick = () => props.onCardClick(node)

  if (isContentNodeWithMedia(node)) {
    const body = node.body
    return (
      <VideoCard
        data={body}
        title={body.name}
        thumbnail={body.thumbnailPath ? convertFileSrc(body.thumbnailPath) : ''}
        onClick={handleClick}
      />
    )
  }

  if (isContentNodeWithList(node)) {
    const body = node.body
    return h(
      props.rendererChildren,
      { data: body },
      () => body.items.map(i => renderContentNodeCard(i, { ...props, rendererChildren: ContentListCard })),
    )
  }

  return h('div', null, ['Not handled'])
}
