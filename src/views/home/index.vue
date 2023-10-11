<template>
  <n-layout class="container-box">
    <n-layout-header class="header-box">
      <TitleBar></TitleBar>
    </n-layout-header>
    <n-layout class="sub-container" has-sider>
      <n-layout-sider class="aside-container">
        <MenuAside @menu-click="menuClick"></MenuAside>
      </n-layout-sider>
      <n-layout-content class="content-container">
        <RouterView v-slot="{ Component }">
          <KeepAlive> <component :is="Component"></component></KeepAlive>
        </RouterView>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>
<script setup lang="ts">
import { NLayout, NLayoutHeader, NLayoutSider, NLayoutContent } from 'naive-ui'
import type { MenuItem } from '../../components/MenuAside/types/index'
import { useRouter } from 'vue-router'

import TitleBar from '../../components/TitleBar/index.vue'
import MenuAside from '../../components/MenuAside/index.vue'

const router = useRouter()
const menuClick = (data: MenuItem) => {
  router.push(`/home/page${data.id}`)
}
</script>

<style scoped>
.container-box {
  width: 100%;
  height: 100%;
}
.sub-container {
  width: 100%;
  height: calc(100% - 30px);
}

.aside-container {
  background-color: #ebebeb;
}
.content-container {
  padding: 25px;
  background-color: #fff;
}
</style>
