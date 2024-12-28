import { defineComposable } from '@webshrine/vue'

export const useUser = defineComposable({
  singleton: true,
  flat: true,
  setup() {
    return {
      name: {
        fill: 'Jane Doe',
        initials: 'JD'
      },
      avatarSrc: '',
    }
  }
})