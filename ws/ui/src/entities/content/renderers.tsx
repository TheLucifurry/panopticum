import type { Component, VNode } from 'vue'
import { ContentNode } from '@panopticum/schemas'
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

  if (ContentNode.isWithMedia(node)) {
    const data = node.data
    return (
      <VideoCard
        data={data}
        title={data.name}
        thumbnail={data.thumbnailPath ? convertFileSrc(data.thumbnailPath) : ''}
        onClick={handleClick}
      />
    )
  }

  if (ContentNode.isWithList(node)) {
    const data = node.data
    return h(
      props.rendererChildren,
      { data, onclick: handleClick },
      () => data.items.map(i => renderContentNodeCard(i, { ...props, rendererChildren: ContentListCard })),
    )
  }

  return h('div', null, ['Not handled'])
}
