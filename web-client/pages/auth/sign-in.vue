<template>
  <div :class="$style.container">
    <AuthSignInContainer v-if="screen === 'welcome'" />
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from "~/store/user.store";

definePageMeta({
  layout: "auth",
});

const authStore = useUserStore();
const router = useRouter();

const screen = computed(() => authStore.authState.screen);

onMounted(async () => {
  if (authStore.user) {
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
