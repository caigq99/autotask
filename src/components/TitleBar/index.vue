<!-- 顶部titlebar -->
<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button" id="titlebar-minimize" @click="minimize">
      <i class="iconfont icon-24gl-minimization"></i>
    </div>
    <div
      v-if="isBig"
      class="titlebar-button"
      id="titlebar-restore"
      @click="restore"
    >
      <i class="iconfont icon-chuangkouhuanyuan"></i>
    </div>
    <div
      v-else
      class="titlebar-button"
      id="titlebar-maximize"
      @click="maximize"
    >
      <i class="iconfont icon-chuangkouzuidahua"></i>
    </div>

    <div class="titlebar-button" id="titlebar-close" @click="close">
      <i class="iconfont icon-guanbi1"></i>
    </div>
  </div>
</template>
<script setup lang="ts">
import { appWindow, LogicalSize } from '@tauri-apps/api/window'
import { ref } from 'vue'

const isBig = ref(false)
const minimize = async () => {
  await appWindow.minimize()
}
const maximize = async () => {
  await appWindow.maximize()
  isBig.value = true
}
const restore = async () => {
  await appWindow.setSize(new LogicalSize(1024, 768))
  await appWindow.center()
  isBig.value = false
}
const close = async () => {
  await appWindow.hide()
  // await appWindow.close()
}
</script>
<style scoped>
.titlebar {
  width: 100%;
  height: 30px;
  background: #ebebeb;
  user-select: none;
  display: flex;
  justify-content: flex-end;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 40px;
  height: 30px;
}
.titlebar-button i {
  font-size: 15px;
  font-weight: bold;
  color: #000;
}

.titlebar-button:hover {
  background: #e5e5e5;
}

#titlebar-close:hover {
  background-color: #e81123;
}
</style>
