<template>
  <div :class="$style.container">
    <p :class="$style.title">Sign-in</p>
    <p :class="$style.subTitle">Sign-in with username and password</p>
    <ControlsInput
      v-model="usernameModel"
      name="username"
      autocomplete="username"
      :error="state.errors.username"
    >
      <template #placeholder>Username</template>
    </ControlsInput>
    <ControlsSpacer :height="20" />
    <ControlsInput
      v-model="passwordModel"
      name="password"
      autocomplete="password"
      type="password"
      :error="state.errors.password"
      >>
      <template #placeholder>Password</template>
    </ControlsInput>
    <ControlsSpacer :height="20" />
    <ControlsButton :is-loading="state.isSubmittingForm" fw @click="onSubmit">
      Sign-in
    </ControlsButton>
    <div v-if="state.errors.submit?.trim() !== ''" :class="$style.error">
      {{ state.errors.submit }}
    </div>
  </div>
</template>

<script lang="js" setup>
import { useAuthStore } from "~/store/auth.store";

const api = useApi();
const authStore = useAuthStore();
const router = useRouter();

const state = reactive({
  form: {
    username: "",
    password: "",
  },
  errors: {
    username: "",
    password: "",
    submit: "",
  },
  isSubmittingForm: false,
});

const usernameModel = computed({
  get: () => state.form.username,
  set: (val) => (state.form.username = val),
});

const passwordModel = computed({
  get: () => state.form.password,
  set: (val) => (state.form.password = val),
});

const onSubmit = async () => {
  if (state.isSubmittingForm) {
    return;
  }

  state.errors.username = "";
  state.errors.password = "";
  state.errors.submit = "";
  state.isSubmittingForm = true;

  if (state.form.username.trim() === "") {
    state.errors.email = "Email can't be empty";
    state.isSubmittingForm = false;
    return;
  }

  if (state.form.password?.trim() === "" || state.form.password === undefined) {
    state.errors.password = "Password is empty";
    state.isSubmittingForm = false;
    return;
  }

  try {
    await sleep(1000);

    await api.auth.login({
      username: state.form.email,
      password: state.form.password,
    });

    authStore.SET_AUTH_STATE({
      screen: "welcome",
      username: state.form.username,
    });

    window.location.href = "/dashboard";
  } catch (e) {
    state.errors.submit = `Error: ${e?.message}`;

    if (e?.status === 400 || e?.status === 401) {
      state.errors.submit = `Username or password is invalid`;
    } else {
      if (e?.status < 500) {
        state.errors.submit = `Error: ${e?.response?.data?.message}`;
      } else {
        state.errors.submit = `Error: server unavailable. Try again later.`;
      }
    }
  } finally {
    state.isSubmittingForm = false;
  }
};
</script>

<style lang="scss" module>
.container {
  width: 100%;
}

.title {
  font-size: 32px;
  margin-bottom: 16px;
}

.subTitle {
  margin-bottom: 16px;
  color: var(--gray-800);
  font-size: 14px;
}

.switchToSignIn {
  width: 100%;
  font-size: 14px;
  color: var(--gray-800);
  text-align: center;

  a {
    color: var(--accent-color-blue);
    text-decoration: underline;
    cursor: pointer;
  }
}

.turnstile {
  margin: 0 auto;
  display: flex;
  justify-content: center;
  height: 72px;
}

.error {
  width: 100%;
  color: var(--status-color-error);
  margin-top: 20px;
}
</style>
