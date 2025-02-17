<script lang="tsx">
import { renderContentNodeCard } from '@/entities/content'
import { PageGrid } from '@/shared/components/custom'
import { usePlayer } from '@/shared/modules'
import { useMediaRepository } from '@/shared/repositories'
import { ContentNode } from '@panopticum/schemas'
import { useAsync } from '@webshrine/vue'
import { defineComponent } from 'vue'
import { useRouter } from 'vue-router'

export default defineComponent({
  emits: ['goLocation'],
  setup(_, { emit }) {
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

    const contentNodes = useAsync(() => mediaRepo.getAllMediaLocal(['Video']), { data: { items: [] } })

    return () => (
      <div class="page-home p-6">
        <PageGrid>
          { contentNodes.value.data.items.map(n => renderContentNodeCard(n, { onCardClick })) }
        </PageGrid>
      </div>
    )
  },
})
</script>

<style lang="scss">
.page-home {
  width: 100%;
  height: 100%;
}
</style>
