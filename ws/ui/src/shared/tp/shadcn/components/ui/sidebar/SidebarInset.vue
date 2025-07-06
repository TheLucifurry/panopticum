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
      't_:bg-background t_:relative t_:flex t_:w-full t_:flex-1 t_:flex-col',
      't_:md:peer-data-[variant=inset]:m-2 t_:md:peer-data-[variant=inset]:ml-0 t_:md:peer-data-[variant=inset]:rounded-xl t_:md:peer-data-[variant=inset]:shadow-sm t_:md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2',
      props.class,
    )"
  >
    <slot />
  </main>
</template>
