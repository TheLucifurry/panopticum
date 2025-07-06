<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/shared/tp/shadcn/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <main
    data-slot="sidebar-inset"
    :class="cn(
      't:bg-background t:relative t:flex t:w-full t:flex-1 t:flex-col',
      't:md:peer-data-[variant=inset]:m-2 t:md:peer-data-[variant=inset]:ml-0 t:md:peer-data-[variant=inset]:rounded-xl t:md:peer-data-[variant=inset]:shadow-sm t:md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2',
      props.class,
    )"
  >
    <slot />
  </main>
</template>
