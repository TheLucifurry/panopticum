import type { ContentNode } from '@panopticum/schemas'
import type { Component, VNode } from 'vue'
import { isContentNodeWithList, isContentNodeWithMedia } from '@panopticum/schemas'
import { convertFileSrc } from '@tauri-apps/api/core'
import { h } from 'vue'
import ContentListCard from './ContentListCard.vue'
import ContentListRow from './ContentListRow.vue'
import VideoCard from './VideoCard.vue'

interface IRenderContentNodeCardProps {
  rendererChildren?: Component
}

export function renderContentNodeCard(node: ContentNode, properties: IRenderContentNodeCardProps = {}): VNode {
  const props: Required<IRenderContentNodeCardProps> = {
    rendererChildren: ContentListRow,
    ...properties,
  }

  if (isContentNodeWithMedia(node)) {
    const body = node.body
    return (
      <VideoCard
        data={body}
        title={body.name}
        thumbnail={body.thumbnailPath ? convertFileSrc(body.thumbnailPath) : ''}
      // onClick={onMediaClick(content)}
      />
    )
  }

  if (isContentNodeWithList(node)) {
    const body = node.body
    return (
      h(
        props.rendererChildren,
        {
          data: body,
        },
        body.items.map(i => renderContentNodeCard(i, { rendererChildren: ContentListCard })),
      )
    )
  }

  return h('div', null, ['Not handled'])
}
