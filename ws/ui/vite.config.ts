import { defineConfig } from "vite";
import Vue from "@vitejs/plugin-vue";
import VueRouter from 'unplugin-vue-router/vite'
import { URL, fileURLToPath } from 'node:url'
import autoprefixer from 'autoprefixer'
import tailwind from 'tailwindcss'
import vueJsx from '@vitejs/plugin-vue-jsx'
import { CSP_DEFAULTS, MetaCSP } from './vite.utils';

// https://github.com/vbenjs/vite-plugin-html
import { createHtmlPlugin as Html } from 'vite-plugin-html'

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    Html({
      inject: {
        tags: [
          MetaCSP({
            ...CSP_DEFAULTS,
            'img-src': `'self' https://*.ytimg.com`,
            'connect-src': `'self' ipc.localhost`,
            // asset: http://asset.localhost
            'media-src': 'http://asset.localhost',
          }),
        ],
      },
    }),
    vueJsx({
      // options are passed on to @vue/babel-plugin-jsx
    }),
    VueRouter({
      root: 'src',
      routeBlockLang: 'yaml',
      routesFolder: 'pages/routes',
      dts: 'pages/typed-router.d.ts',
    }),
    Vue(),
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
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
  },
  css: {
    postcss: {
      plugins: [tailwind(), autoprefixer()],
    },
    preprocessorOptions: {
      scss: {
        api: 'modern-compiler'
      }
    }
  }
});
