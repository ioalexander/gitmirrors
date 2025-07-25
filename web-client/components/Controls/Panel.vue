<template>
  <div :class="$style.panel">
    <slot />
  </div>
</template>

<style lang="scss" module>
.panel {
  background: var(--black-transparent);
  backdrop-filter: var(--black-transparent-backdrop);
  filter: var(--shadow-large);

    padding: 20px;
  border-radius: 16px;
  border: 1px solid var(--gray-900);
}
</style>
