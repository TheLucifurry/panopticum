import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'
import { computed } from 'vue'
import { AnyObject, omit } from 'webshrine'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

const omitClass = <T extends AnyObject>(v: T) => omit(v, ['class'] as const)

export const useDelegatedProps = <T extends AnyObject>(getter: () => T) => computed(() => omitClass<T>(getter()))