import { defineComposable } from '@webshrine/vue'
import { shallowRef } from 'vue'

const DEFAULT_VOLUME = 0.5

export const usePlayer = defineComposable({
  singleton: true,
  flat: true,
  setup() {
    const volume = shallowRef(DEFAULT_VOLUME)

    return {
      volume,
      currentTime: 25,
      trackLengthTime: 500
    }
  }
})