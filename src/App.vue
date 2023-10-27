<template>
  <NConfigProvider class="config-provider-container" :theme="theme">
    <NDialogProvider>
      <NNotificationProvider>
        <NMessageProvider>
          <RouterView></RouterView>
          <NThemeEditor></NThemeEditor>
        </NMessageProvider>
      </NNotificationProvider>
    </NDialogProvider>
  </NConfigProvider>
</template>
<script setup lang="ts">
import { NConfigProvider, NDialogProvider, NNotificationProvider, NMessageProvider, NThemeEditor } from "naive-ui";
import { computed } from "vue";
const theme = computed(() => {
  return undefined;
});
</script>
<style scoped>
.config-provider-container {
  height: 100%;
  border-radius: 50px;
}
</style>
