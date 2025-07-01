<template></template>

<script setup lang="ts">
import { useAuthStore } from "~/store/auth.store";

definePageMeta({
  layout: "auth",
});

const authStore = useAuthStore();
const router = useRouter();

onMounted(async () => {
  if (authStore.session) {
    await router.push("/dashboard");
  } else {
    await router.push("/auth/sign-in");
  }
});
</script>
