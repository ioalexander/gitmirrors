<template>
  <div :class="[$style.input, { [$style.readonly]: props.readonly }]">
    <div v-if="!state.isPlaceholderHidden" :class="$style.placeholderContainer">
      <slot name="placeholder" />
    </div>
    <div
      :class="$style.htmlInputContainer"
      :style="computedHtmlInputContainerStyle"
    >
      <textarea
        ref="inputRef"
        :autocomplete="props.autocomplete"
        :type="props.type"
        :name="props.name"
        :rows="props.rows"
        :class="$style.htmlInput"
        @focus="handleFocus"
        @blur="handleBlur"
        @input="handleInput"
        @keyup.enter="handleEnter"
      ></textarea>
    </div>
    <div v-if="isError" :class="$style.error">{{ props.error }}</div>
  </div>
</template>

<script setup lang="ts">
const inputRef = ref<HTMLInputElement | null>(null);

const emit = defineEmits(["focus", "blur", "enter"]);

const props = defineProps({
  name: {
    type: String,
  },
  type: {
    type: String,
    default: "text",
  },
  autocomplete: {
    type: String,
  },
  error: {
    type: String,
  },
  readonly: {
    type: Boolean,
    default: false,
  },
  rows: {
    type: Number,
    default: 3,
  },
});

const model = defineModel<string>();

const state = reactive({
  isFocused: false,
  isPlaceholderHidden: false,
  isFilled: false,
});

const computedHtmlInputContainerStyle = computed(() => {
  const rows = props.rows ?? 1;
  return {
    height: `${rows * 20 + 32}px`,
  };
});

const isError = computed(() => !!props.error && props.error?.trim() !== "");

const handleFocus = () => {
  state.isFocused = true;
  state.isPlaceholderHidden = true;
  emit("focus");
};

const handleBlur = () => {
  state.isFocused = false;

  if (!state.isFilled) {
    state.isPlaceholderHidden = false;
  }

  emit("blur");
};

const handleInput = (e: any) => {
  model.value = e.target?.value;

  state.isFilled = inputRef?.value?.value?.length !== 0;
  state.isPlaceholderHidden = state.isFilled;
};

const handleEnter = () => {
  emit("enter");
};

const setValue = (value: string) => {
  if (!inputRef?.value) {
    return;
  }

  inputRef.value.value = value;
};

onMounted(async () => {
  const existingValue = inputRef?.value?.value;
  state.isFilled = existingValue?.length !== 0;
  state.isPlaceholderHidden = state.isFilled;
  model.value = existingValue;
});

defineExpose({ setValue });
</script>

<style lang="scss" module>
.input {
  width: 100%;
  position: relative;

  &.readonly {
    pointer-events: none;
  }

  .placeholderContainer {
    position: absolute;
    left: calc(16px + 2px);
    top: calc(48px / 2);
    transform: translateY(-50%);
    pointer-events: none;
    color: var(--gray-800);
  }

  .htmlInputContainer {
    width: 100%;
    height: 48px;
    border-radius: 8px;
    border: 1px solid var(--gray-900);
    background: var(--black-transparent);
    backdrop-filter: var(--black-transperent-backdrop);

    .htmlInput {
      width: 100%;
      height: 100%;
      padding: 16px;
      resize: none;
      background: none;
      border: none;
      outline: 0;
    }
  }

  .error {
    margin-top: 10px;
    color: var(--status-color-error);
  }
}
</style>
