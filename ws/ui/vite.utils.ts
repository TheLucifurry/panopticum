import type { HtmlTagDescriptor } from 'vite'

type TCspKeys = 'default-src' | 'script-src' | 'style-src' | 'img-src' | 'font-src' | 'connect-src' | 'media-src' | 'object-src' | 'manifest-src' | 'frame-src' | 'child-src'

type TCspOptions = Partial<Record<TCspKeys | (string & {}), string>>
export const SELF = `'self'`

function createCSPString(cspOptions: TCspOptions): string {
  return Object.entries(cspOptions)
    .map(([key, value]) => `${key} ${value}`)
    .join(';')
}

export const Tag = (tag: keyof HTMLElementTagNameMap, attrs: Record<string, string>): HtmlTagDescriptor => ({ tag, attrs })
export function MetaCSP(cspOptions: TCspOptions) {
  return Tag('meta', {
    'http-equiv': 'Content-Security-Policy',
    'content': createCSPString(cspOptions),
  })
}
