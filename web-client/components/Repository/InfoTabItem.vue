<template>
  <li :class="$style.item" @click="copyToClipboard">
    <div :class="$style.label">
      <Icon :class="$style.icon" :name="icon" />
      {{ label }}
    </div>
    <div :class="$style.value">
      <slot />
    </div>
  </li>
</template>

<script setup lang="ts">
import { useSlots } from "vue";
import { useToast } from "vue-toastification";

defineProps<{
  icon: string;
  label: string;
}>();

const slots = useSlots();
const toast = useToast();

const copyToClipboard = async () => {
  const value = slots.default?.()[0].children;
  if (typeof value === "string") {
    try {
      await navigator.clipboard.writeText(value);
      toast.success("Value copied to clipboard!");
    } catch (err) {
      toast.error("Failed to copy value to clipboard!");
    }
  }
};
</script>

<style lang="scss" module>
.item {
  min-width: 0;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--gray-900);
  border-radius: 8px;
  padding: 8px;
  user-select: none;
  cursor: pointer;

  &:hover {
    background: var(--black-transparent-hover);
  }

  .label {
    color: var(--gray-600);
    font-size: 14px;

    .icon {
      margin-right: 4px;
      transform: translate(0px, 1px);
      color: var(--gray-600);
      font-size: 14px;
    }
  }

  .value {
    color: var(--white);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    /* direction: rtl; */
    text-align: left;
  }
}
</style>
