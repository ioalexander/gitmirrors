<template>
  <div>
    {{ user }}
  </div>
</template>
<script lang="ts">
import { useUserStore } from "~/store/user.store";

const userStore = useUserStore();

const user = computed(() => userStore.user);
</script>
