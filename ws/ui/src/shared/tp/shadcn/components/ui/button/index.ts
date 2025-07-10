import type { VariantProps } from 'class-variance-authority'
import { cva } from 'class-variance-authority'

export { default as Button } from './Button.vue'

export const buttonVariants = cva(
  't:inline-flex t:items-center t:justify-center t:gap-2 t:whitespace-nowrap t:rounded-md t:text-sm t:font-medium t:transition-all t:disabled:pointer-events-none t:disabled:opacity-50 t:[&_svg]:pointer-events-none t:[&_svg:not([class*=\'size-\'])]:size-4 t:shrink-0 t:[&_svg]:shrink-0 t:outline-none t:focus-visible:border-ring t:focus-visible:ring-ring/50 t:focus-visible:ring-[3px] t:aria-invalid:ring-destructive/20 t:dark:aria-invalid:ring-destructive/40 t:aria-invalid:border-destructive',
  {
    variants: {
      variant: {
        default:
          't:bg-primary t:text-primary-foreground t:shadow-xs t:hover:bg-primary/90',
        destructive:
          't:bg-destructive t:text-white t:shadow-xs t:hover:bg-destructive/90 t:focus-visible:ring-destructive/20 t:dark:focus-visible:ring-destructive/40 t:dark:bg-destructive/60',
        outline:
          't:border t:bg-background t:shadow-xs t:hover:bg-accent t:hover:text-accent-foreground t:dark:bg-input/30 t:dark:border-input t:dark:hover:bg-input/50',
        secondary:
          't:bg-secondary t:text-secondary-foreground t:shadow-xs t:hover:bg-secondary/80',
        ghost:
          't:hover:bg-accent t:hover:text-accent-foreground t:dark:hover:bg-accent/50',
        link: 't:text-primary t:underline-offset-4 t:hover:underline',
      },
      size: {
        default: 't:h-9 t:px-4 t:py-2 t:has-[>svg]:px-3',
        sm: 't:h-8 t:rounded-md t:gap-1.5 t:px-3 t:has-[>svg]:px-2.5',
        lg: 't:h-10 t:rounded-md t:px-6 t:has-[>svg]:px-4',
        icon: 't:size-9',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
)

export type ButtonVariants = VariantProps<typeof buttonVariants>
