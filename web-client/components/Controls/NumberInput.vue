<template>
  <div :class="[$style.input, { [$style.readonly]: props.readonly }]">
    <div v-if="!state.isPlaceholderHidden" :class="$style.placeholderContainer">
      <slot name="placeholder" />
    </div>
    <div :class="$style.htmlInputContainer">
      <input
        ref="inputRef"
        :autocomplete="props.autocomplete"
        type="number"
        :name="props.name"
        :class="$style.htmlInput"
        :max="props.max"
        :min="props.min"
        :step="props.step"
        @focus="handleFocus"
        @blur="handleBlur"
        @input="handleInput"
        @keyup.enter="handleEnter"
      >
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
    default: "",
  },
  autocomplete: {
    type: String,
    default: "",
  },
  error: {
    type: String,
    default: "",
  },
  readonly: {
    type: Boolean,
    default: false,
  },
  min: {
    type: Number,
    default: 0,
  },
  max: {
    type: Number,
    default: 999999,
  },
  step: {
    type: Number,
    default: 1,
  },
  defaultValue: {
    type: Number,
    default: 0,
  },
});

const model = defineModel<number>();

const state = reactive({
  isFocused: false,
  isPlaceholderHidden: false,
  isFilled: false,
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

  if (inputRef.value) {
    inputRef.value.value = Number.parseFloat(
      model?.value?.toString() as string,
    )?.toString();
  }

  emit("blur");
};

const handleInput = (e: Event) => {
  const target = e.target as HTMLInputElement;
  let val = Number(target.value);

  if (isNaN(val)) {
    model.value = 0;
    state.isFilled = false;
    state.isPlaceholderHidden = false;
    return;
  }

  if (props.min !== undefined && val < props.min) val = props.min;
  if (props.max !== undefined && val > props.max) val = props.max;

  if (props.step !== undefined && props.step > 0) {
    val = Math.round(val / props.step) * props.step;
  }

  model.value = val;

  state.isFilled = target.value.length !== 0;
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
  if (!inputRef.value) {
    return;
  }

  if (props.defaultValue) {
    inputRef.value.value = props.defaultValue?.toString();
  }

  const existingValue = inputRef?.value?.value;
  state.isFilled = existingValue?.length !== 0;
  state.isPlaceholderHidden = state.isFilled;
  model.value = Number.parseFloat(existingValue as string);
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
    background: var(--black-transparent);
    border: 1px solid var(--gray-900);

    .htmlInput {
      width: 100%;
      height: 100%;
      padding: 0 16px;
    }
  }

  .error {
    margin-top: 10px;
    color: var(--status-color-error);
  }
}
</style>
