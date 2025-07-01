<template>
  <div :class="$style.container">
    <AuthSignInContainer v-if="screen === 'welcome'" />
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from "~/store/auth.store";

definePageMeta({
  layout: "auth",
});

const authStore = useAuthStore();
const router = useRouter();

const screen = computed(() => authStore.authState.screen);

onMounted(async () => {
  if (authStore.session) {
    await router.push("/dashboard");
  }
});
</script>

<style lang="scss" module>
.container {
  width: 100%;
  display: grid;
  grid-template-columns: 1fr;
  grid-gap: 20px;
  padding: 32px;
}
</style>
