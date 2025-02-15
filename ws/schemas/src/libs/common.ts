export type ValueLabelPair = [value: string, label: string]

export type PathNode = string | ValueLabelPair

export namespace PathNode {
  export const isValueLabelPair = Array.isArray as (v: PathNode) => v is ValueLabelPair
  export const toValue = (v: PathNode) => typeof v === 'string' ? v : v[0]
  export const toLabel = (v: PathNode) => typeof v === 'string' ? v : v[1]
}

export type PathNodes = PathNode[]
