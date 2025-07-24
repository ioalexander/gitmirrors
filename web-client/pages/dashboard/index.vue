<template>
  <Topbar>
    <div :class="$style.topbarContent">
      <h1 :class="$style.title">Dashboard</h1>
    </div>
  </Topbar>
  <div :class="$style.container"></div>
</template>
<script setup lang="ts">
import { useUserStore } from "~/store/user.store";

const userStore = useUserStore();
const user = computed(() => userStore.user);
</script>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;
}

.topbarContent {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  .title {
    font-size: 20px;
  }
  .right {
    display: flex;
  }
}
</style>
