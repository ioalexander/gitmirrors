<template>
  <div>User: {{ user }}</div>
</template>
<script setup lang="ts">
import { useUserStore } from "~/store/user.store";

const userStore = useUserStore();
const user = computed(() => userStore.user);
</script>
