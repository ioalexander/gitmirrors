<template>
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
</style>
