
export function toDurationStringFromSeconds(seconds: number | undefined): string {
  return seconds != null ? new Date(1000 * seconds).toISOString().slice(14, 19) : '--:--'
}