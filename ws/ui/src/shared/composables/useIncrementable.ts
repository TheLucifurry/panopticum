import type { Ref } from 'vue'
import { clampNumber } from 'webshrine'

interface IRefIncrementableOptions {
  step?: number
  max?: number
  min?: number
}

export function useIncrementable<T extends number = number>(targetTef: Ref<T, T>, options: IRefIncrementableOptions = {}) {
  const { step = 1, max = Number.MAX_SAFE_INTEGER, min = Number.MIN_SAFE_INTEGER } = options

  const inc = (stepMultiplier = 1) => {
    targetTef.value = clampNumber(targetTef.value + step * stepMultiplier, min, max) as T
  }

  return {
    step,
    max,
    min,
    inc,
    dec: (stepMultiplier = -1) => inc(stepMultiplier),
  } as const
}
