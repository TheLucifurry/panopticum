/* eslint-disable */
/* prettier-ignore */
// @ts-nocheck
// Generated by unplugin-vue-router. ‼️ DO NOT MODIFY THIS FILE ‼️
// It's recommended to commit this file.
// Make sure to add this file to your tsconfig.json file as an "includes" or "files" entry.

declare module 'vue-router/auto-routes' {
  import type {
    RouteRecordInfo,
    ParamValue,
    ParamValueOneOrMore,
    ParamValueZeroOrMore,
    ParamValueZeroOrOne,
  } from 'vue-router'

  /**
   * Route name map generated by unplugin-vue-router
   */
  export interface RouteNamedMap {
    'player': RouteRecordInfo<'player', '/', Record<never, never>, Record<never, never>>,
    '404': RouteRecordInfo<'404', '/:all(.*)', { all: ParamValue<true> }, { all: ParamValue<false> }>,
    'home': RouteRecordInfo<'home', '/home', Record<never, never>, Record<never, never>>,
    'provider': RouteRecordInfo<'provider', '/p/:id', { id: ParamValue<true> }, { id: ParamValue<false> }>,
  }
}
