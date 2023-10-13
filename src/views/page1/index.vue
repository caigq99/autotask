<!--  -->
<template>
  <div class="page">
    <n-result status="404" title="404 资源不存在" description="生活总归带点荒谬">
      <template #footer>
        <n-button>找点乐子吧</n-button>
      </template>
    </n-result>
  </div>
</template>
<script setup lang="ts">
import { NResult, NButton } from "naive-ui";
</script>

<style scoped>
/* .page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
} */
</style>
