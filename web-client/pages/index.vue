<template></template>

<script setup lang="ts">
definePageMeta({
  layout: "auth",
});

const router = useRouter();

onMounted(async () => {
  await router.push("/auth/sign-in");
});
</script>
