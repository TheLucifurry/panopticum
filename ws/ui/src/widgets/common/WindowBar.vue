<script lang="tsx">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Minus, PanelTop, X } from 'lucide-vue-next';
import { defineComponent, VNode } from 'vue';
import { cope, Fn } from 'webshrine';

const [appWindow] = cope(getCurrentWindow)

export default defineComponent({
  setup(_, ctx) {
    const renderButton = (callback: Fn, content: VNode) => (
      <div class="window-bar__button" onClick={callback}>
        {content}
      </div>
    )

    return () => (
      <div class="window-bar">
        <div>
          {ctx.slots.default?.()}
        </div>
        <div data-tauri-drag-region></div>
        <div>
          {ctx.slots.extra?.()}
          {renderButton(() => appWindow?.minimize(), (
            <Minus/>
          ))}
          {renderButton(() => appWindow?.toggleMaximize(), (
            <PanelTop/>
          ))}
          {renderButton(() => appWindow?.close(), (
            <X/>
          ))}
        </div>
      </div>
    )
  }
})
</script>

<style lang="scss">
$height: 40px;

.window-bar {
  background-color: #0001;
  height: $height;
  width: 100%;
  user-select: none;
  display: grid;
  top: 0;
  left: 0;
  right: 0;
  grid-template-columns: max-content 1fr max-content;

  &__button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: $height;
    height: $height;
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
