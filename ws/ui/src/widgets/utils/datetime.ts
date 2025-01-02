
const toTwoDigitMin = (s: string | number) => s.toString().padStart(2, '0')
const ceil = Math.trunc

export function toDurationStringFromSeconds(seconds: number | undefined): string {
  return seconds != null
    ? `${ceil(seconds / 3600)}:${toTwoDigitMin(ceil(seconds / 60))}:${toTwoDigitMin(ceil(seconds % 60))}`
    : "--:--"
  // return seconds != null ? new Date(1000 * seconds).toISOString().slice(14, 19) : '--:--'
}