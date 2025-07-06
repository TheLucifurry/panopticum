<script lang="tsx">
import type { ComponentObjectPropsOptions } from 'vue'
import { defineComponent } from 'vue'
import Avatar from '@/shared/tp/shadcn/components/ui/avatar/Avatar.vue'

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
