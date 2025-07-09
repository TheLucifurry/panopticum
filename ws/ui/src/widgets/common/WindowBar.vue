<script lang="tsx">
import type { FunctionalComponent } from 'vue'
import type { Fn } from 'webshrine'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { MinusIcon, PanelTopIcon, XIcon } from 'lucide-vue-next'
import { defineComponent, h } from 'vue'
import { cope } from 'webshrine'

const [win] = cope(getCurrentWindow)

export default defineComponent({
  setup(_, ctx) {
    const renderButton = (callback: Fn, content: FunctionalComponent) => (
      <div class="button" onClick={callback}>
        {h(content, { size: 16 })}
      </div>
    )

    const minimize = () => win?.minimize()
    const toggleMaximize = () => win?.toggleMaximize()
    const close = () => win?.close()

    return () => (
      <div class="window-bar">
        <div>
          {ctx.slots.default?.()}
        </div>
        <div data-tauri-drag-region></div>
        <div>
          {ctx.slots.extra?.()}
          {renderButton(minimize, MinusIcon)}
          {renderButton(toggleMaximize, PanelTopIcon)}
          {renderButton(close, XIcon)}
        </div>
      </div>
    )
  },
})
</script>

<style>
.window-bar {
  --window-bar__height: 40px;

  background-color: #0001;
  height: var(--window-bar__height);
  width: 100%;
  user-select: none;
  display: grid;
  top: 0;
  left: 0;
  right: 0;
  grid-template-columns: max-content 1fr max-content;

  .button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: var(--window-bar__height);
    height: var(--window-bar__height);
    user-select: none;
    -webkit-user-select: none;

    &:hover {
      background: #0001;
    }
    &:active {
      background: #0002;
    }
  }

  & > div {
    display: flex;
    align-items: center;
  }
}
</style>
