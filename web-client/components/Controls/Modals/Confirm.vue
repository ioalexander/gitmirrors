<template>
  <Base :is-open="model">
    <div :class="$style.title">
      <slot name="title" />
    </div>
    <div :class="$style.content">
      <slot name="content" />
    </div>
    <div :class="$style.buttons">
      <ControlsButton is-white @click="emit('cancel')">Cancel</ControlsButton>
      <ControlsButton :is-red="isRed" @click="emit('confirm')">
        Confirm
      </ControlsButton>
    </div>
  </Base>
</template>

<script setup lang="ts">
import Base from "./Base.vue";

const model = defineModel<boolean>();

defineProps({
  isRed: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["cancel", "confirm"]);
</script>

<style lang="scss" module>
.title {
  font-size: 30px;
  margin-bottom: 20px;
  width: 500px;
}
.content {
  width: 500px;
  margin-bottom: 20px;

  p {
    margin-bottom: 10px;
  }
}
.buttons {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
