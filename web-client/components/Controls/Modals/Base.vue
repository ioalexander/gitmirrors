<template>
  <Teleport to="#teleports">
    <div :class="[$style.modal, { [$style.isOpen]: props.isOpen }]">
      <div :class="$style.window">
        <slot />
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false,
  },
});
</script>

<style lang="scss" module>
.modal {
  width: 100%;
  height: 100%;
  position: fixed;
  left: 0;
  top: 0;
  background: rgb(0, 0, 0, 0);
  transition: 0.5s;
  pointer-events: none;
  display: flex;
  justify-content: center;
  align-items: center;

  &.isOpen {
    background: rgb(0, 0, 0, 0.2);
    backdrop-filter: var(--black-transparent-backdrop);
    pointer-events: all;

    .window {
      opacity: 1;
      transform: translateY(0px);
    }
  }

  .window {
    opacity: 0;
    transition: 0.5s;
    transform: translateY(-20px);
    background: var(--black-transparent);
    backdrop-filter: var(--black-transparent-backdrop);
    border: 1px solid var(--gray-900);
    padding: 20px;
    margin: auto;
    border-radius: 16px;
  }
}
</style>
