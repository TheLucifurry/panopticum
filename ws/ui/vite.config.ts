import process from 'node:process'
import { fileURLToPath, URL } from 'node:url'
import tailwindcss from '@tailwindcss/vite'
import Vue from '@vitejs/plugin-vue'
import VueJsx from '@vitejs/plugin-vue-jsx'
import autoprefixer from 'autoprefixer'
import VueMacros from 'unplugin-vue-macros/vite'
import VueRouter from 'unplugin-vue-router/vite'
import { defineConfig } from 'vite'
// https://github.com/vbenjs/vite-plugin-html
import { createHtmlPlugin as Html } from 'vite-plugin-html'

import { MetaCSP, SELF } from './vite.utils'

const ENV = {
  host: process.env.TAURI_DEV_HOST,
  isWindowsTarget: process.env.TAURI_ENV_PLATFORM === 'windows',
  isDebug: !!process.env.TAURI_ENV_DEBUG,
} as const

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    Html({
      inject: {
        tags: [
          MetaCSP({
            'default-src': `${SELF} ipc: http://ipc.localhost`,
            'script-src': SELF,
            'style-src': `${SELF} 'unsafe-inline'`,
            'img-src': `${SELF} https://*.ytimg.com asset: http://asset.localhost`,
            'connect-src ipc': `${SELF} http://ipc.localhost`,
            'media-src': `${SELF} http://asset.localhost`,
          }),
        ],
      },
    }),
    VueMacros({
      plugins: {
        vue: Vue(),
        vueJsx: VueJsx(),
        vueRouter: VueRouter({
          root: 'src',
          routeBlockLang: 'yaml',
          routesFolder: 'pages',
          dts: 'app/typed-router.d.ts',
          extensions: ['.vue', '.setup.tsx'],
        }),
      },
    }),
    tailwindcss(),
  ],

  // Custom
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: ENV.host || false,
    hmr: ENV.host
      ? {
          protocol: 'ws',
          host: ENV.host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/core/src/**'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  css: {
    postcss: {
      plugins: [
        autoprefixer(),
      ],
    },
    preprocessorOptions: {
      scss: {
        // api: 'modern-compiler',
      },
    },
  },
  build: {
    target: ENV.isWindowsTarget ? 'chrome105' : 'safari13', // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    minify: ENV.isDebug ? false : 'esbuild',
    sourcemap: ENV.isDebug,
  },
})
