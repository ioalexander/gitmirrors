<template>
  <div :class="$style.log">
    <div
      :class="[
        $style.icon,
        {
          [$style.starting_clone_job]: props.log.type === 'starting_clone_job',
          [$style.finished_clone_job]: props.log.type === 'finished_clone_job',
          [$style.error_clone_job]: props.log.type === 'error_clone_job',
        },
      ]"
    >
      <Icon
        v-if="props.log.type === 'starting_clone_job'"
        name="material-symbols:pending"
      />
      <Icon
        v-if="props.log.type === 'finished_clone_job'"
        name="weui:done-filled"
      />
      <Icon
        v-if="props.log.type === 'error_clone_job'"
        name="material-symbols:exclamation-rounded"
      />

      <div :class="$style.line"/>
    </div>
    <div :class="$style.info">
      <div :class="$style.text">{{ props.log.message }}</div>
      <div :class="$style.time">{{ formattedDate }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RepositoryLog } from "~/types/repositoryLog";

const props = defineProps<{
  log: RepositoryLog;
}>();

const formattedDate = computed(() => {
  const date = new Date(props.log.createdAt); // Adjust field as needed
  return date
    .toLocaleString("en-US", {
      month: "short",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    })
    .replace(",", "");
});
</script>

<style lang="scss" module>
.log {
  width: 100%;
  margin-bottom: 20px;

  display: grid;
  grid-template-columns: 30px 1fr;
  grid-gap: 20px;

  .icon {
    width: 30px;
    height: 30px;
    background: var(--black-200-transparent);
    border-radius: 100%;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;

    .line {
      position: absolute;
      left: 50%;
      bottom: -20px;
      transform: translateX(-50%);
      width: 1px;
      height: 0px;
      animation: grow-line 0.5s ease-out 1s forwards;
      background: var(--gray-900);
    }

    &.starting_clone_job {
      background: var(--status-color-pending);
    }
    &.finished_clone_job {
      background: var(--status-color-success);
    }
    &.error_clone_job {
      background: var(--status-color-error);
    }
  }

  .info {
    width: 100%;
    display: grid;
    grid-template-columns: 1fr 100px;

    .text {
      width: 100%;
    }
    .time {
      width: 100%;
      color: var(--gray-900);
      font-size: 14px;
    }
  }
}

@keyframes grow-line {
  from {
    height: 0;
  }
  to {
    height: 20px;
  }
}
</style>
