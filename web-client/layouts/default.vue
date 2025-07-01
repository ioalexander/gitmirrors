<template>
  <div :class="$style.container">
    <aside :class="$style.sidebar">
      <NuxtLink to="/">Home</NuxtLink>
    </aside>
    <div :class="$style.content">
      <slot />
    </div>
  </div>
</template>

<style lang="scss" module>
.container {
  width: 100%;
  display: grid;
  grid-template-columns: 200px;
  grid-gap: 0;

  .sidebar {
    width: 100%;
    border-right: 1px solid gray;
  }

  .content {
    width: 100%;
  }
}
</style>
