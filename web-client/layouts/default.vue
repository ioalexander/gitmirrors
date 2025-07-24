<template>
  <div :class="[$style.container, { [$style.fold]: isSidebarFold }]">
    <Sidebar :class="$style.sidebar" />
    <div :class="$style.content">
      <div :class="$style.transition">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useUiStore } from "~/store/ui.store";

const uiStore = useUiStore();

const isSidebarFold = computed(() => uiStore.isSidebarFold);
</script>

<style lang="scss" module>
.container {
  width: calc(100%);
  padding-left: 300px;
  display: block;
  transition: 0.5s;

  .sidebar {
    width: 300px;
    height: 100%;
    position: fixed;
    left: 0;
    top: 0;
    transition: 0.5s;
  }

  .content {
    width: 100%;
    min-height: 100vh;
    padding: 30px;
  }
}

.container.fold {
  padding-left: 80px;

  .sidebar {
    width: 80px;
  }
}
</style>
