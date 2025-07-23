<template>
  <div :class="$style.item">
    <div :class="$style.info">
      <h2 :class="$style.title">{{ repository.name }}</h2>
      <a :class="$style.url" :href="repository.url" target="_blank">{{
        repository.url
      }}</a>
    </div>
    <div :class="$style.actions">
      <Button is-gray> View </Button>
    </div>
  </div>
</template>
<script setup lang="ts">
import Button from "~/components/Controls/Button.vue";
import type { Repository } from "~/types/repository";

defineProps<{
  repository: Repository;
}>();
</script>
<style lang="scss" module>
.item {
  padding: 30px;
  border: 1px solid var(--gray-900);
  border-radius: 16px;

  .info {
    margin-bottom: 30px;

    .title {
      font-size: 20px;
      margin-bottom: 4px;
    }

    .url {
      font-size: 14px;
    }
  }
}
</style>
