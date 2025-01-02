
const toTwoDigitMin = (s: string | number) => s.toString().padStart(2, '0')
const ceil = Math.trunc

const SECS_IN_HOUR = 60 * 60

export function toDurationStringFromSeconds(seconds: number | undefined): string {
  return seconds != null
    ? `${seconds < SECS_IN_HOUR ? '' : ceil(seconds / 3600) + ':'}${toTwoDigitMin(ceil(seconds / 60))}:${toTwoDigitMin(ceil(seconds % 60))}`
    : "--:--"
}