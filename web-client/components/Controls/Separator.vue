<template>
  <div :style="{ height: height / 2 + 'px' }" />
  <div :class="$style.line"/>
  <div :style="{ height: height / 2 + 'px' }" />
</template>

<script>
export default {
  props: {
    height: {
      type: Number,
      default: 20,
    },
  },
};
</script>

<style lang="scss" module>
.line {
  width: 100%;
  max-width: 200px;
  background: var(--gray-200);
  height: 1px;
  margin: 0 auto;
}
</style>
