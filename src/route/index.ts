import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    name: 'home',
    path: '/',
    redirect: () => '/home',
    children: [
      {
        name: 'home',
        path: 'home',
        component: () => import('../views/home/index.vue'),
        children: [
          {
            name: 'page1',
            path: 'page1',
            component: () => import('../views/page1/index.vue'),
          },
          {
            name: 'page2',
            path: 'page2',
            component: () => import('../views/page2/index.vue'),
          },
          {
            name: 'page3',
            path: 'page3',
            component: () => import('../views/page3/index.vue'),
          },
          {
            name: 'page4',
            path: 'page4',
            component: () => import('../views/page4/index.vue'),
          },
          {
            name: 'page5',
            path: 'page5',
            component: () => import('../views/page5/index.vue'),
          },
        ],
      },
    ],
  },
]

const router = createRouter({
  routes,
  history: createWebHistory(),
})

export default router
