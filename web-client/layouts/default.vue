<template>
  <div :class="$style.container">
    <Sidebar :class="$style.sidebar" />
    <div :class="$style.content">
      <div :class="$style.transition">
        <slot />
      </div>
    </div>
  </div>
</template>

<style lang="scss" module>
.container {
  width: calc(100%);
  padding-left: 300px;
  display: block;

  .sidebar {
    width: 300px;
    height: 100%;
    position: fixed;
    left: 0;
    top: 0;
  }

  .content {
    width: 100%;
    min-height: 100vh;
    padding: 30px;
  }
}
</style>
