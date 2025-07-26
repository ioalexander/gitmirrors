<template>
  <div :class="$style.container">
    <Input
      v-model="passwordModel"
      type="password"
      name="password"
      autocomplete="new-password"
      :error="state.errors.password"
    >
      <template #placeholder>Enter new password</template>
    </Input>

    <Input
      v-model="confirmPasswordModel"
      type="password"
      name="confirm password"
      autocomplete="new-password"
      :error="state.errors.confirmPassword"
    >
      <template #placeholder>Confirm password</template>
    </Input>

    <Button :is-loading="state.isSubmittingForm" fw @click="onSubmit">
      Change Password
    </Button>
    <div v-if="state.errors.submit?.trim() !== ''" :class="$style.error">
      {{ state.errors.submit }}
    </div>
  </div>
</template>

<script setup lang="ts">
import Input from "../Controls/Input.vue";
import Button from "../Controls/Button.vue";
import { isAxiosError } from "axios";

const api = useApi();

const state = reactive({
  form: {
    password: "",
    confirmPassword: "",
  },
  errors: {
    password: "",
    confirmPassword: "",
    submit: "",
  },
  isSubmittingForm: false,
});

const passwordModel = computed({
  get: () => state.form.password,
  set: (val) => (state.form.password = val),
});

const confirmPasswordModel = computed({
  get: () => state.form.confirmPassword,
  set: (val) => (state.form.confirmPassword = val),
});

const onSubmit = async () => {
  if (state.isSubmittingForm) {
    return;
  }

  state.errors.password = "";
  state.errors.confirmPassword = "";
  state.errors.submit = "";
  state.isSubmittingForm = true;

  if (state.form.password.trim() === "") {
    state.errors.password = "Password can't be empty";
    state.isSubmittingForm = false;
    return;
  }

  if (state.form.password?.trim() === "" || state.form.password === undefined) {
    state.errors.password = "Password is empty";
    state.isSubmittingForm = false;
    return;
  }

  if (state.form.password !== state.form.confirmPassword) {
    state.errors.confirmPassword = "Passwords don't match!";
    state.isSubmittingForm = false;
    return;
  }

  try {
    await sleep(1000);

    const response = await api.user.changePassword({
      password: state.form.password,
    });

    if (response.success !== true) {
      throw new Error("Malformed Response");
    }
  } catch (e: unknown) {
    console.log("Error happened", e);

    let status: number | undefined;
    let message = "Unknown error";

    if (e instanceof Error) {
      message = e.message;

      if (isAxiosError(e)) {
        status = e.response?.status;
        message = e.response?.data?.message ?? message;
      }
    }

    if (status === 400 || status === 401 || status === 404) {
      state.errors.submit = `Username or password is invalid`;
    } else if (status !== undefined && status < 500) {
      state.errors.submit = `Error: ${message}`;
    } else if (status !== undefined) {
      state.errors.submit = `Error: server unavailable. Try again later.`;
    } else {
      state.errors.submit = `Error: ${message}`;
    }
  } finally {
    state.isSubmittingForm = false;
  }
};
</script>

<style lang="scss" module>
.container {
  max-width: 400px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
</style>
