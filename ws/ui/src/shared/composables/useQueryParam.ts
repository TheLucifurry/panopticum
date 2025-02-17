import type { MaybeRef, Ref } from 'vue'
import type { LocationQueryValue } from 'vue-router'
import type { AnyObject } from 'webshrine'
import { ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { cope, isNullish } from 'webshrine'

type TQueryParams = Record<string, LocationQueryValue | LocationQueryValue[]>
type TQueryParamsValues = string | Record<string, unknown> | Array<unknown>

export function getEncodedQueryParams(params: Record<string, TQueryParamsValues>): TQueryParams {
  return Object.entries(params)
    .reduce((acc, [key, value]) => {
      if (typeof value === 'string') {
        acc[key] = encodeURIComponent(decodeURIComponent(value))
      }
      else {
        const [data] = cope(() => JSON.stringify(value))
        if (!isNullish(data))
          acc[key] = encodeURIComponent(data)
      }

      return acc
    }, {} as TQueryParams)
}

type TUseQueryParamMode = 'get' | 'extract' | 'sync'

// TODO: вынести конвертор, когда понадобится
const STRING_TO_STRING_CONVERTOR: IStringConvertor = {
  parse: v => v,
  stringify: v => `${v}`,
}
const JSON_TO_STRING_CONVERTOR: IStringConvertor = {
  parse: (v: string) => {
    const [data = null] = cope(() => JSON.parse(v) as AnyObject)
    return data
  },
  stringify: v => JSON.stringify(v),
}

type TStringConvertorConstructor = JSON | StringConstructor
interface IStringConvertor<T = unknown> {
  parse: (value: string) => T | null
  stringify: (value: T) => string
}
function getStringConvertor<T>(convertor?: TStringConvertorConstructor): IStringConvertor<T> {
  switch (convertor) {
    case JSON: return JSON_TO_STRING_CONVERTOR as IStringConvertor<T>
    default: return STRING_TO_STRING_CONVERTOR as IStringConvertor<T>
  }
}

/**
 * Использует параметр строки URL по ключу
 * @example ```
 * // Режим экстракта и гета
 *  const searchString = useQueryParam('search', 'extract'); // Вернет значение параметра и удалит его из URL
 *  const searchString = useQueryParam('search', 'get'); // Вернет значение параметра без удаления его из URL
 * ```
 * @example ```
 * // Режим синхронизации
 *  const item = useQueryParam('item', 'sync'); // Вернет реф с изначальным значением из URL
 *  item.value = 'another_item' // Обновит параметр в URL
 * ```
 */
export function useQueryParam<Value = string>(key: string, mode: 'get', convertor?: TStringConvertorConstructor): Value | null
export function useQueryParam<Value = string>(key: string, mode: 'extract', convertor?: TStringConvertorConstructor): Value | null
export function useQueryParam<Value = string>(key: string, mode: 'sync', convertor?: TStringConvertorConstructor): Ref<Value | null>
export function useQueryParam<Value = string>(
  key: string,
  mode: TUseQueryParamMode,
  convertor?: TStringConvertorConstructor,
): MaybeRef<Value | null> {
  const convert = getStringConvertor<Value>(convertor)

  const router = useRouter()
  const route = router.currentRoute.value
  const valueRaw = route.query[key]
  const valueDecoded = typeof valueRaw === 'string' ? decodeURIComponent(valueRaw) : null
  const valueParsed = valueDecoded ? convert.parse(valueDecoded) : null

  if (mode === 'extract' || mode === 'get') {
    if (mode === 'extract') {
      delete route.query[key]
    }

    return valueParsed
  }

  const value = ref(valueParsed) as Ref<Value | null>

  watch(value, (nevValue) => {
    if (!isNullish(nevValue)) {
      router.replace({
        query: {
          ...route.query,
          ...getEncodedQueryParams({
            [key]: convert.stringify(nevValue),
          }),
        },
      })
    }
    else {
      delete route.query[key]
    }
  }, { deep: true })

  return value
}
