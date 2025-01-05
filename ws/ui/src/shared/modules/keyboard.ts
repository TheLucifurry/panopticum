import { defineModule } from '@webshrine/vue'
import keyboardJS from 'keyboardjs'
import { onScopeDispose } from 'vue'

export const useKeyboard = defineModule(() => {
  return {
    init(target = window) {
      keyboardJS.watch(target)
    },
    bind(keyCombo: string | string[], pressed: keyboardJS.Callback | null, released?: keyboardJS.Callback, preventRepeatByDefault?: boolean) {
      keyboardJS.bind(keyCombo, pressed, released, preventRepeatByDefault)
      onScopeDispose(() => keyboardJS.unbind(keyCombo, pressed, released))
    },
  }
})
