<!--  -->
<template>
  <div class="page">
    <n-result status="403" title="403 禁止访问" description="总有些门是对你关闭的">
      <template #footer>
        <n-button>放轻松</n-button>
      </template>
    </n-result>
  </div>
</template>
<script setup lang="ts">
import { NResult, NButton } from "naive-ui";
</script>

<style scoped></style>
