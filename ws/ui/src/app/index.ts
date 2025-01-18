import { createApp } from 'vue'
import { debug } from './debug'
import App from './index.vue'
import Router from './router'
// import './styles/index.scss'
import './styles/global.css'

if (import.meta.env.DEV)
  debug()

createApp(App)
  .use(Router)
  .mount('#root')
