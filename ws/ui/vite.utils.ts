import type { HtmlTagDescriptor } from 'vite'

type TCspOptions = Partial<Record<TCspKeys, string>>
export const CSP_DEFAULTS: TCspOptions = {
  'default-src': `'self'`,
  'script-src': `'self'`,
  'style-src': `'self' 'unsafe-inline'`,
}

type TCspKeys = 'default-src' | 'script-src' | 'style-src' | 'img-src' | 'font-src' | 'connect-src' | 'media-src' | 'object-src' | 'manifest-src' | 'frame-src' | 'child-src'

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
