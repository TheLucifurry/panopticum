import Router from '@/pages'
import { createApp } from 'vue'
import { debug } from './debug'
import App from './index.vue'
// import './styles/index.scss'
import './styles/global.css'

if (import.meta.env.DEV)
  debug()

createApp(App)
  .use(Router)
  .mount('#root')
