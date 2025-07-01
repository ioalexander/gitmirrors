<template>
  <div :style="{ height: height + 'px' }" />
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
