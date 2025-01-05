<template>
  <div class="z-bezel bg-black bg-opacity-05 py-3 px-5 text-xl rounded-md">
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
