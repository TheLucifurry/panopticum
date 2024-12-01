import { createApp } from "vue";
import { createPinia } from 'pinia'
import App from "./index.vue";
import { debug } from './debug'
import Router from '@/pages'
import './styles/index.scss'

if (import.meta.env.DEV)
  debug()

const Pinia = createPinia()

createApp(App)
  .use(Router)
  .use(Pinia)
  .mount("#root")
