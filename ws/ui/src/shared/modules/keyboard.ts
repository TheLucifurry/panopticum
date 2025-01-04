import { defineComposable } from '@webshrine/vue'
import keyboardJS from 'keyboardjs'
import { onScopeDispose } from 'vue'

export const useKeyboard = defineComposable({
  singleton: true,
  flat: true,
  setup() {
    return {
      init(target = window) {
        keyboardJS.watch(target)
      },
      bind(keyCombo: string | string[], pressed: keyboardJS.Callback | null, released?: keyboardJS.Callback, preventRepeatByDefault?: boolean) {
        keyboardJS.bind(keyCombo, pressed, released, preventRepeatByDefault)
        onScopeDispose(() => keyboardJS.unbind(keyCombo, pressed, released))
      },
    }
  },
})
