<script lang="tsx">
import type { ContentNode } from '@panopticum/schemas'
import { renderContentNodeCard } from '@/entities/content'
import { PageGrid } from '@/shared/components/custom'
import { usePlayer } from '@/shared/modules'
import { useMediaRepository } from '@/shared/repositories'
import { isContentNodeWithMedia } from '@panopticum/schemas'
import { useAsync } from '@webshrine/vue'
import { defineComponent } from 'vue'
import { useRouter } from 'vue-router'

export default defineComponent({
  setup() {
    const router = useRouter()
    const player = usePlayer()
    const mediaRepo = useMediaRepository()

    function onCardClick(node: ContentNode) {
      if (isContentNodeWithMedia(node)) {
        player.setCurrentMedia(node.body)
        router.push({ name: 'player' })
        player.togglePlaying(true)
      }
    }

    const contentNodes = useAsync(mediaRepo.getAllMediaLocal, [])

    return () => (
      <div class="page-home p-6">
        <PageGrid>
          { contentNodes.value.map(n => renderContentNodeCard(n, { onCardClick })) }
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
