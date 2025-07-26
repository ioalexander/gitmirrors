<template>
  <ControlsPanel :class="$style.item">
    <div :class="$style.info">
      <h2 :class="$style.title">{{ repository.name }}</h2>
      <a :class="$style.url" :href="repository.url" target="_blank">
        <span :class="$style.val">
          {{ repository.url.replace(/^https?:\/\//, "") }}
        </span>
        <Icon :class="$style.icon" name="ci:external-link" />
      </a>
    </div>
    <div :class="$style.actions">
      <NuxtLink
        :class="$style.view"
        :to="`/dashboard/repository/${props.repository.id}`"
      >
        View
        <Icon
          :class="$style.buttonArrowIcon"
          name="material-symbols:arrow-forward-rounded"
        />
      </NuxtLink>
    </div>
  </ControlsPanel>
</template>
<script setup lang="ts">
import type { Repository } from "~/types/repository";

const props = defineProps<{
  repository: Repository;
}>();
</script>
<style lang="scss" module>
.item {
  width: 100%;
  height: 200px;
  padding: 30px;
  border: 1px solid var(--gray-900);
  border-radius: 16px;
  min-width: 0;

  .info {
    margin-bottom: 30px;

    .title {
      font-size: 20px;
      margin-bottom: 4px;
    }

    .url {
      font-size: 14px;
      display: flex;

      .val {
        color: var(--accent);
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }

      .icon {
        min-width: 16px;
        color: var(--accent);
        transform: translate(2px, 3px);
      }
    }
  }

  .actions {
    display: flex;
    justify-content: flex-end;

    .view {
      background: var(--black-200-transparent);
      padding: 10px 20px;
      border-radius: 8px;
      color: var(--white);
      text-decoration: none;
      border: 1px solid var(--gray-900);

      &:hover {
        filter: brightness(110%);
      }

      .buttonArrowIcon {
        transform: translate(5px, 2px);
      }
    }
  }
}
</style>
