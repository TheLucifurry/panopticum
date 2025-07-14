<script lang="tsx">
import type { PathNodes } from '@panopticum/schemas'
import type { PropType } from 'vue'
import { ContentNode } from '@panopticum/schemas'
import { useAsync } from '@webshrine/vue'
import { defineComponent, watch } from 'vue'
import { useRouter } from 'vue-router'
import { renderContentNodeCard } from '@/entities/content'
import { usePlayer } from '@/modules/player'
import { PageGrid } from '@/shared/components/custom'
import { useMediaRepository } from '@/shared/repositories'

export default defineComponent({
  props: {
    locationPath: {
      type: Object as PropType<PathNodes>,
    },
  },
  emits: ['goLocation'],
  setup(props, { emit }) {
    const router = useRouter()
    const player = usePlayer()
    const mediaRepo = useMediaRepository()

    function onCardClick(node: ContentNode) {
      if (ContentNode.isWithMedia(node)) {
        player.setCurrentMedia(node.data)
        router.push({ name: 'player' })
        player.togglePlaying(true)
      }
      if (ContentNode.isWithList(node)) {
        emit('goLocation', [node.data.name])
      }
    }

    const contentNodes = useAsync(() => mediaRepo.getAllMediaLocal(props.locationPath), { data: { items: [] } })

    watch(() => props.locationPath, () => contentNodes.execute())

    return () => (
      <div class="page-home t:p-6">
        {contentNodes.error.value
          ? (
              <div> Error</div>
            )
          : null}
        <PageGrid>
          { contentNodes.value.data.items.map(n => renderContentNodeCard(n, { onCardClick })) }
        </PageGrid>
      </div>
    )
  },
})
</script>

<style>
.page-home {
  width: 100%;
  height: 100%;
}
</style>
