<template>
  <div :class="[$style.spacer, { [$style.fold]: isSidebarFold }]">
    <header :class="$style.header">
      <slot />
    </header>
  </div>
</template>

<script setup lang="ts">
import { useUiStore } from "~/store/ui.store";

const uiStore = useUiStore();

const isSidebarFold = computed(() => uiStore.isSidebarFold);
</script>

<style lang="scss" module>
.spacer {
  width: 100%;
  height: 80px;

  .header {
    z-index: 1000;
    padding: 0 30px;
    display: flex;
    align-items: center;
    width: calc(100% - 300px);
    border-bottom: 1px solid var(--gray-900);
    position: fixed;
    left: 300px;
    top: 0;
    height: 80px;
    background: var(--black-transparent);
    backdrop-filter: var(--black-transparent-backdrop);
    filter: var(--shadow-large);
    transition: 0.5s;
  }
}

.spacer.fold {
  .header {
    width: calc(100% - 80px);
    left: 80px;
  }
}
</style>
