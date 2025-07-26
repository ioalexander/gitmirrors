<template>
  <div :class="$style.error">
    <h1 :class="$style.code">{{ error?.statusCode }}</h1>
    <p :class="$style.message">{{ error?.statusMessage }}</p>
    <NuxtLink :class="$style.goBackHome" to="/">Go back home</NuxtLink>
  </div>
</template>

<script setup lang="ts">
import type { NuxtError } from "#app";

const props = defineProps<{
  error?: NuxtError;
}>();

onMounted(() => {
  console.error("ERROR", props?.error);
});
</script>

<style lang="scss" module>
.error {
  width: 100%;
  height: 100vh;

  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;

  .code {
    font-size: 64px;
    margin-bottom: 10px;
  }

  .goBackHome {
    display: flex;
    justify-content: center;
    align-items: center;
    background: var(--accent-color-blue);
    color: var(--white);
    text-decoration: none;
    padding: 10px 20px;
    border-radius: 10px;
    margin-bottom: 10px;
  }

  .message {
    text-align: center;
    color: var(--gray-600);
    margin-bottom: 30px;
  }
}
</style>
