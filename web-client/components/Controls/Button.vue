<template>
  <button
    :class="[
      {
        [$style.isWhite]: props.isWhite,
        [$style.isRed]: props.isRed,
        [$style.fw]: props.fw,
        [$style.isGray]: props.isGray,
      },
    ]"
    @click="emit('click')"
  >
    <Icon
      v-if="props.isLoading"
      :class="$style.svgIcon"
      name="line-md:loading-loop"
    />

    <div
      :class="$style.content"
      :style="{
        opacity: props.isLoading ? 0 : 1,
      }"
    >
      <slot />
    </div>
  </button>
</template>

<script setup lang="ts">
const emit = defineEmits(["click"]);

const props = defineProps({
  fw: {
    type: Boolean,
    default: false,
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
  isGray: {
    type: Boolean,
    default: false,
  },
  isWhite: {
    type: Boolean,
    default: false,
  },
  isRed: {
    type: Boolean,
    default: false,
  },
});
</script>

<style lang="scss" module>
.svgIcon {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 20px;
  height: 20px;
  pointer-events: none;
}

.isRed {
  background: var(--red);
  color: var(--white);

  .content {
    color: var(--white);
  }
}

.isGray {
  background: var(--black-100);
  color: var(--white);

  .content {
    color: var(--white);
  }
}

.isWhite {
  background: var(--white);
  color: var(--black);

  .content {
    color: var(--black);
  }
}

.fw {
  width: 100%;
}
</style>
