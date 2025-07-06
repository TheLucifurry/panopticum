import { cva, type VariantProps } from 'class-variance-authority'

export { default as Button } from './Button.vue'

export const buttonVariants = cva(
  't_:inline-flex t_:items-center t_:justify-center t_:gap-2 t_:whitespace-nowrap t_:rounded-md t_:text-sm t_:font-medium t_:transition-all t_:disabled:pointer-events-none t_:disabled:opacity-50 t_:[&_svg]:pointer-events-none t_:[&_svg:not([class*=\'size-\'])]:size-4 t_:shrink-0 t_:[&_svg]:shrink-0 t_:outline-none t_:focus-visible:border-ring t_:focus-visible:ring-ring/50 t_:focus-visible:ring-[3px] t_:aria-invalid:ring-destructive/20 t_:dark:aria-invalid:ring-destructive/40 t_:aria-invalid:border-destructive',
  {
    variants: {
      variant: {
        default:
          't_:bg-primary t_:text-primary-foreground t_:shadow-xs t_:hover:bg-primary/90',
        destructive:
          't_:bg-destructive t_:text-white t_:shadow-xs t_:hover:bg-destructive/90 t_:focus-visible:ring-destructive/20 t_:dark:focus-visible:ring-destructive/40 t_:dark:bg-destructive/60',
        outline:
          't_:border t_:bg-background t_:shadow-xs t_:hover:bg-accent t_:hover:text-accent-foreground t_:dark:bg-input/30 t_:dark:border-input t_:dark:hover:bg-input/50',
        secondary:
          't_:bg-secondary t_:text-secondary-foreground t_:shadow-xs t_:hover:bg-secondary/80',
        ghost:
          't_:hover:bg-accent t_:hover:text-accent-foreground t_:dark:hover:bg-accent/50',
        link: 't_:text-primary t_:underline-offset-4 t_:hover:underline',
      },
      size: {
        default: 't_:h-9 t_:px-4 t_:py-2 t_:has-[>svg]:px-3',
        sm: 't_:h-8 t_:rounded-md t_:gap-1.5 t_:px-3 t_:has-[>svg]:px-2.5',
        lg: 't_:h-10 t_:rounded-md t_:px-6 t_:has-[>svg]:px-4',
        icon: 't_:size-9',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
)

export type ButtonVariants = VariantProps<typeof buttonVariants>
