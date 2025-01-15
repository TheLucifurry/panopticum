type EventElement = HTMLElement | Window | Document
type EventName<T> = T extends Window
  ? keyof WindowEventMap
  : T extends Document ? keyof DocumentEventMap : keyof HTMLElementEventMap
type EventListener = (this: HTMLElement, ev: any) => unknown
type EventOptions = boolean | AddEventListenerOptions

export function on(e: EventElement, eventName: EventName<EventElement>, listener: EventListener, options?: EventOptions) {
  e.addEventListener(eventName, listener, options)
}

export function once(e: EventElement, eventName: EventName<EventElement>, listener: EventListener, options: Omit<AddEventListenerOptions, 'once'> = {}) {
  e.addEventListener(eventName, listener, { ...options, once: true })
}

export function off(e: EventElement, eventName: EventName<EventElement>, listener: EventListener, options?: EventOptions) {
  e.removeEventListener(eventName, listener, options)
}
