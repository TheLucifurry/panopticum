import { createRouter, createWebHistory } from 'vue-router/auto'
import { routes } from 'vue-router/auto-routes'

const Router = createRouter({
  history: createWebHistory(),
  routes,
})

export default Router
