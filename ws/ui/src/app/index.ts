import Router from '@/pages'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import { debug } from './debug'
import App from './index.vue'
// import './styles/index.scss'
import './styles/global.css'

if (import.meta.env.DEV)
  debug()

const Pinia = createPinia()

createApp(App)
  .use(Router)
  .use(Pinia)
  .mount('#root')
