<!--  -->
<template>
  <div class="page">
    <n-result status="404" title="大！大！啊" description="这么大" size="huge">
      <template #footer>
        <n-button>哦</n-button>
      </template>
    </n-result>
  </div>
</template>
<script setup lang="ts">
import { NResult, NButton } from "naive-ui";
</script>

<style scoped></style>
