<template>
  <div :class="$style.spacer">
    <header :class="$style.header">
      <slot />
    </header>
  </div>
</template>

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
  }
}
</style>
