<template>
  <div class="z-bezel t:bg-black t:bg-black/50 t:py-3 t:px-5 t:text-xl t:rounded-md">
    <slot />
  </div>
</template>

<style>
.z-bezel {
  position: absolute;
  transform: translateX(-50%);
  left: 50%;
  top: 10%;
  color: #fff;
}
</style>
