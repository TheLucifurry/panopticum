export function debug() {
  const ignoreErrors = [
    'ResizeObserver loop limit exceeded',
    'ResizeObserver loop completed with undelivered notifications.',
  ]

  const showErrorOverlay = (err: ErrorEvent | string | unknown) => {
    if (!err)
      return

    const error = typeof err === 'string' ? { message: err } : err as ErrorEvent
    const isSkip = ignoreErrors.includes(error.message)

    if (isSkip)
      return

    // must be within function call because that's when the element is defined for sure.
    const ErrorOverlay = customElements.get('vite-error-overlay')
    // don't open outside vite environment
    if (!ErrorOverlay)
      return

    const overlay = new ErrorOverlay()
    document.body.appendChild(overlay)
    console.error('[UNHANDLED ERROR]:', err)
  }

  window.addEventListener('error', showErrorOverlay)
  window.addEventListener('unhandledrejection', ({ reason }: PromiseRejectionEvent) => showErrorOverlay(reason))
}
