import { useInteraction } from '@/shared/modules/interaction'
import { defineModule } from '@webshrine/vue'
import keyboardJS from 'keyboardjs'
import { onScopeDispose } from 'vue'
import { noop, pipe } from 'webshrine'

type KeyCombo = string
interface KeyboardBindOptions {
  pressed: keyboardJS.Callback | null
  released?: keyboardJS.Callback
  preventRepeatByDefault?: boolean
  bezel?: () => any
}

type KeyboardBindList = Record<KeyCombo, KeyboardBindOptions | keyboardJS.Callback>

export const useKeyboard = defineModule(() => {
  const i10 = useInteraction()

  return {
    init(target = window) {
      keyboardJS.watch(target)
    },
    binds(binds: KeyboardBindList) {
      const unbindList: Array<[KeyCombo, KeyboardBindOptions['pressed'], KeyboardBindOptions['released']]> = []

      for (const [keyCombo, options] of Object.entries(binds)) {
        const { pressed, released, preventRepeatByDefault, bezel } = typeof options === 'function' ? { pressed: options } : options

        const bezelShower = bezel ? () => i10.bezel.show(bezel()) : noop
        const onPress = pressed ? pipe(pressed, bezelShower) : noop
        const onRelease = released ? pipe(released, bezelShower) : noop

        keyboardJS.bind(keyCombo, onPress, onRelease, preventRepeatByDefault)
        unbindList.push([keyCombo, onPress, onRelease])
      }

      onScopeDispose(() => unbindList.forEach(args => keyboardJS.unbind(...args)))
    },
  }
})
