<!-- 左侧功能菜单 -->
<template>
  <div class="menu-aside-container">
    <ul class="menu-container">
      <li
        :class="['menu-item', index === item.id ? 'select' : '']"
        v-for="item in menuArr"
        :key="item.id"
        @click="itemClickHandle(item)"
      >
        {{ item.name }}
      </li>
    </ul>
  </div>
</template>
<script setup lang="ts">
import { onMounted, ref, reactive, nextTick } from 'vue'
import { MenuItem } from './types/index'
const emit = defineEmits<{
  (event: 'menu-click', data: MenuItem): void
}>()

const index = ref<number>(1)
const menuArr = reactive<MenuItem[]>([
  { name: '常规', id: 1 },
  { name: '代理', id: 2 },
  { name: '配置', id: 3 },
  { name: '日志', id: 4 },
  { name: '连接', id: 5 },
])
let menulist = ref<NodeListOf<HTMLElement>>()

// tab项点击事件处理
const itemClickHandle = (item: MenuItem) => {
  index.value = item.id
  setBorderRadius(menulist.value as NodeListOf<HTMLElement>, item.id - 1)
  emit('menu-click', item)
}

// 在点击tab切换时设置圆角
const setBorderRadius = (menulist: NodeListOf<HTMLElement>, index: number) => {
  nextTick(() => {
    menulist.forEach((item) => {
      item.classList.remove('radius-top-right', 'radius-bottom-right')
    })
    if (menulist[index - 1]) {
      menulist[index - 1].classList.add('radius-bottom-right')
    }
    if (menulist[index + 1]) {
      menulist[index + 1].classList.add('radius-top-right')
    }
  })
}

onMounted(() => {
  menulist.value = document.querySelectorAll('.menu-item')
  itemClickHandle(menuArr[0])
})
</script>
<style scoped>
.menu-aside-container {
  width: 100%;
}
.menu-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  background-color: #fff;
}

.menu-item {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 60px;
  background-color: #ebebeb;
  color: #666;
  font-size: 17px;
  cursor: pointer;
}
.menu-item.select {
  color: #333;
  background-color: #fff;
}

.radius-top-right {
  border-top-right-radius: 15px;
  overflow: hidden;
}
.radius-bottom-right {
  border-bottom-right-radius: 15px;
  overflow: hidden;
}
</style>
