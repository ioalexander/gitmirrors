<template>
  <div :class="$style.container">
    <div :class="$style.list">
      <slot />
    </div>
  </div>
</template>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;

  .list {
    width: 100%;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    grid-gap: 30px;

    @media all and (max-width: 1800px) {
      & {
        grid-template-columns: 1fr 1fr 1fr;
      }
    }

    @media all and (max-width: 1800px) {
      & {
        grid-template-columns: 1fr 1fr;
      }
    }
  }
}
</style>
