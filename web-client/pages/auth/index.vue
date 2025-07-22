<template></template>

<script setup lang="ts">
import { useUserStore } from "~/store/user.store";

definePageMeta({
  layout: "auth",
});

const authStore = useUserStore();
const router = useRouter();

onMounted(async () => {
  if (authStore.user) {
    await router.push("/dashboard");
  } else {
    await router.push("/auth/sign-in");
  }
});
</script>
