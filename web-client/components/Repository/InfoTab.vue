<template>
  <section :class="$style.container">
    <h2 :class="$style.name">{{ props.repository?.name }}</h2>
    <ul :class="$style.additionalInfo">
      <li :class="$style.item">
        <div :class="$style.label">UUID:</div>
        <div :class="$style.value">{{ props.repository?.id }}</div>
      </li>
      <li :class="$style.item">
        <div :class="$style.label">Url:</div>
        <div :class="$style.value">{{ props.repository?.url }}</div>
      </li>
      <li :class="$style.item">
        <div :class="$style.label">Created at:</div>
        <div :class="$style.value">{{ formattedCreatedAt }}</div>
      </li>
      <li :class="$style.item">
        <div :class="$style.label">git Source:</div>
        <div :class="$style.value">{{ props.repository?.gitSource }}</div>
      </li>
      <li :class="$style.item">
        <div :class="$style.label">git Target:</div>
        <div :class="$style.value">{{ props.repository?.gitTarget }}</div>
      </li>
    </ul>
  </section>
</template>

<script setup lang="ts">
import type { Repository } from "~/types/repository";

const props = defineProps<{
  repository?: Repository;
}>();
const formattedCreatedAt = computed(() => {
  if (!props.repository?.createdAt) return "";
  const date = new Date(props.repository.createdAt);
  const months = [
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
  ];
  return `${date.getFullYear()} ${months[date.getMonth()]} ${String(date.getDate()).padStart(2, "0")} ${String(date.getHours()).padStart(2, "0")}:${String(date.getMinutes()).padStart(2, "0")}`;
});
</script>

<style lang="scss" module>
.container {
  width: 100%;
  padding: 20px;
  border-radius: 16px;
  border: 1px solid var(--gray-900);

  .name {
    font-size: 20px;
    margin-bottom: 20px;
  }

  .additionalInfo {
    list-style: none;

    .item {
      display: grid;
      grid-template-columns: 100px 1fr;

      & > * {
        color: var(--gray-600);
        font-size: 14px;
      }
    }
  }
}
</style>
