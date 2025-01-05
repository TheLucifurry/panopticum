import { defineModule } from '@webshrine/vue'

export const useUser = defineModule(() => {
  return {
    name: {
      full: 'Jane Doe',
      initials: 'JD',
    },
    avatarSrc: '',
  }
})
