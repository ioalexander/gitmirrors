<template>
  <div>{{ repositoryId }}</div>
</template>
<script setup lang="ts">
const route = useRoute();
const repositoryId = computed(() => route.params.id);
</script>
