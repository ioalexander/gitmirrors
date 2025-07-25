<template>
  <ControlsPanel :class="$style.container">
    <TransitionGroup name="itemappear" tag="div" :class="$style.animatedList">
      <RepositoryLogsTabLog
        v-for="log in props.logs"
        :key="log.id"
        :log="log"
        class="item"
      />
    </TransitionGroup>
  </ControlsPanel>
</template>

<script setup lang="ts">
import type { RepositoryLog } from "~/types/repositoryLog";

const props = defineProps<{
  logs: RepositoryLog[];
}>();
</script>

<style lang="scss" module>
.container {
  width: 100%;
}

.animatedList {
  height: 400px;
  overflow: hidden;
  display: block;
  position: relative;
}

/* spacing between items */
.animatedList .item + .item {
  margin-top: 8px;
}
</style>

<style lang="scss">
.itemappear-enter-active,
.itemappear-leave-active {
  transition: all 0.3s ease;
}
.itemappear-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}
.itemappear-enter-to {
  opacity: 1;
  transform: translateY(0);
}
.itemappear-leave-from {
  opacity: 1;
  transform: translateY(0);
}
.itemappear-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

/* animate positional shifts */
.itemappear-move {
  transition: transform 1s ease;
}
</style>
