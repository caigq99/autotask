<template>
  <NLayout class="container-box">
    <NLayoutHeader class="header-box">
      <TitleBar></TitleBar>
    </NLayoutHeader>
    <NLayout class="sub-container" has-sider>
      <NLayoutSider class="aside-container">
        <MenuAside @menu-click="menuClick"></MenuAside>
      </NLayoutSider>
      <NLayoutContent class="content-container">
        <RouterView v-slot="{ Component }">
          <KeepAlive> <component :is="Component"></component></KeepAlive>
        </RouterView>
      </NLayoutContent>
    </NLayout>
  </NLayout>
</template>
<script setup lang="ts">
import { NLayout, NLayoutHeader, NLayoutSider, NLayoutContent } from "naive-ui";
import type { MenuItem } from "../../components/MenuAside/types/index";
import { useRouter } from "vue-router";

import TitleBar from "../../components/TitleBar/index.vue";
import MenuAside from "../../components/MenuAside/index.vue";

const router = useRouter();
const menuClick = (data: MenuItem) => {
  router.push(`/home/page${data.id}`);
};
</script>

<style scoped>
.container-box {
  height: 100%;
}
.sub-container {
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
