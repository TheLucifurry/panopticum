import { createApp } from 'vue'
import App from './index.vue'
import Router from './router'
import './styles/index.css'

if (import.meta.env.DEV) {
  import('./debug').then(({ debug }) => debug())
}

createApp(App)
  .use(Router)
  .mount('#a')
