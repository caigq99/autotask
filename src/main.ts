import { createApp } from 'vue'
import App from './App.vue'
import router from './route/index'

import './assets/fonts/iconfont.css'
import './assets/css/global.css'

createApp(App).use(router).mount('#app')
