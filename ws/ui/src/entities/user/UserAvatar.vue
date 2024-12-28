<script lang="tsx">
import Avatar from '@/shared/components/ui/avatar/Avatar.vue';
import { ComponentObjectPropsOptions, defineComponent } from 'vue'

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
  }
})
</script>
