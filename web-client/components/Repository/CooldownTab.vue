<template>
  <ControlsPanel :class="$style.container">
    <h2 :class="$style.name">Next clone in:</h2>
    <div :class="$style.clockContainerPadding">
      <div :class="$style.clockContainer">
        <div :class="$style.text">{{ cloneDueFormatted }}</div>
        <svg :class="$style.svg" width="100" height="100" viewBox="0 0 100 100">
          <circle
            cx="50"
            cy="50"
            r="45"
            fill="none"
            stroke="#fff"
            stroke-width="10"
            stroke-dasharray="282.6"
            :stroke-dashoffset="
              circumference *
              ((props?.cloneDue as number) <= 0 ? 1 : clockPieProgress)
            "
            transform="rotate(-90 50 50)"
          />
        </svg>
      </div>
    </div>
  </ControlsPanel>
</template>

<script setup lang="ts">
import type { Repository } from "~/types/repository";

const circumference = 2 * Math.PI * 45;
const props = defineProps<{
  repository?: Repository;
  cloneDue?: number;
}>();

const clockPieProgress = computed(() => {
  if (!props.cloneDue) {
    return 0;
  }

  const now = Date.now();
  const last = new Date(props.repository?.lastCloneAt ?? 0).getTime();
  const elapsed = Math.max(now - last, 0);
  const remaining = Math.max(props.cloneDue * 1000, 0);
  const totalInterval = elapsed + remaining;
  const ratio = totalInterval > 0 ? elapsed / totalInterval : 0;
  return Math.min(Math.max(ratio, 0), 1);
});

const cloneDueFormatted = computed(() => {
  if (!props.cloneDue) {
    return "Soon...";
  }
  const totalSeconds = Math.floor(props.cloneDue);
  const hours = String(Math.floor(totalSeconds / 3600)).padStart(2, "0");
  const minutes = String(Math.floor((totalSeconds % 3600) / 60)).padStart(
    2,
    "0",
  );
  const seconds = String(totalSeconds % 60).padStart(2, "0");

  return `${hours}:${minutes}:${seconds}`;
});
</script>

<style lang="scss" module>
.container {
  width: 100%;
  padding: 20px;
  border-radius: 16px;
  border: 1px solid var(--gray-900);
}
.name {
  font-size: 20px;
  margin-bottom: 20px;
}
.clockContainerPadding {
  width: 100%;
  min-height: 200px;
  max-height: 500px;
  position: relative;

  .clockContainer {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;

    .text {
      position: absolute;
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
    }

    .svg {
      position: absolute;
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
      width: 100%;
      height: 100%;

      circle {
        stroke: var(--accent-color-blue);
        transition: stroke-dashoffset 0.5s ease;
      }
    }
  }
}
</style>
