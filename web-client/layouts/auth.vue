<template>
  <div :class="$style.authPage">
    <div :class="$style.container">
      <slot />
    </div>
  </div>
</template>
<style lagng="scss" module>
.authPage {
  width: 100%;
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

.container {
  width: 100%;
  max-width: 500px;
  background: var(--white);
  filter: var(--shadow);
  border-radius: 16px;
}
</style>
