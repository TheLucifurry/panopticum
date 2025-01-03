<script lang="tsx">
import type { ComponentObjectPropsOptions } from 'vue'
import Avatar from '@/shared/components/ui/avatar/Avatar.vue'
import { defineComponent } from 'vue'

const props: ComponentObjectPropsOptions = {

}

export default defineComponent({
  props,
  setup(props, ctx) {
    return () => (
      <Avatar>
        {ctx.slots.default?.()}
      </Avatar>
    )
  },
})
</script>
