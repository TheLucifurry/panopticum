export type ValueLabelPair = [value: string, label: string]

export type PathNode = string | ValueLabelPair

export type PathNodes = PathNode[]

export const isPathNodeAValueLabelPair = Array.isArray as (v: PathNode) => v is ValueLabelPair