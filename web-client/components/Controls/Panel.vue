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
}
</style>
