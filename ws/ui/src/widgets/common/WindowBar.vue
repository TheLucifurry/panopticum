<script lang="tsx">
import { getCurrentWindow } from '@tauri-apps/api/window';
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
          {renderButton(() => appWindow?.minimize(), (
            <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
          ))}
          {renderButton(() => appWindow?.toggleMaximize(), (
            <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
          ))}
          {renderButton(() => appWindow?.close(), (
            <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
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
