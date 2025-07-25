<template>
  <ControlsPanel :class="$style.container">
    <h2 :class="$style.name">{{ props.repository?.name }}</h2>
    <ul :class="$style.additionalInfo">
      <InfoTabItem icon="material-symbols:grid-3x3" label="UUID">
        {{ props.repository?.id }}
      </InfoTabItem>
      <InfoTabItem icon="streamline:link-chain" label="Url">
        {{ props.repository?.url }}
      </InfoTabItem>
      <InfoTabItem icon="material-symbols:alarm-add" label="Created at">
        {{ formattedCreatedAt }}
      </InfoTabItem>
      <InfoTabItem icon="material-symbols:frame-source" label="git Source">
        {{ props.repository?.gitSource }}
      </InfoTabItem>
      <InfoTabItem icon="mdi:target" label="git Target">
        {{ props.repository?.gitTarget }}
      </InfoTabItem>
    </ul>
  </ControlsPanel>
</template>

<script setup lang="ts">
import type { Repository } from "~/types/repository";
import InfoTabItem from "./InfoTabItem.vue";

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

  .name {
    font-size: 20px;
    margin-bottom: 20px;
  }

  .additionalInfo {
    list-style: none;
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 10px;
  }
}
</style>
