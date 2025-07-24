<template>
  <ControlsModalsBase :is-open="model">
    <h2 :class="$style.title">Add new Repository</h2>
    <div :class="$style.content">
      <div :class="$style.columns">
        <ControlsInput v-model="nameModel" :error="state.errors.name"
          ><template #placeholder>Name</template></ControlsInput
        >
        <ControlsInput v-model="urlModel" :error="state.errors.url"
          ><template #placeholder>Url (optional)</template></ControlsInput
        >
      </div>
      <ControlsSpacer :height="40" />
      <!-- -->
      <div :class="$style.columns">
        <ControlsInput v-model="gitSourceModel" :error="state.errors.gitSource"
          ><template #placeholder>git Source</template></ControlsInput
        >
        <ControlsInput v-model="gitTargetModel" :error="state.errors.gitTarget"
          ><template #placeholder>git Target</template></ControlsInput
        >
        <ControlsTextareaInput
          v-model="gitSourceSecretKeyModel"
          :rows="7"
          :error="state.errors.gitSourceSecretKey"
          ><template #placeholder
            >git Source Secret Key (optional)</template
          ></ControlsTextareaInput
        >

        <!-- -->

        <ControlsTextareaInput
          v-model="gitTargetSecretKeyModel"
          :rows="7"
          :error="state.errors.gitTargetSecretKey"
          ><template #placeholder
            >git Target Secret Key</template
          ></ControlsTextareaInput
        >
      </div>
      <ControlsSpacer :height="40" />
      <!-- -->
      <div :class="$style.columns">
        <ControlsNumberInput
          v-model="cloningPeriodModel"
          :error="state.errors.cloningPeriod"
          :min="60"
          :max="31556952"
          :step="60"
          :default-value="60 * 5"
          ><template #placeholder
            >Cloning period (seconds)</template
          ></ControlsNumberInput
        >
        <div :class="$style.cloneDisplay">
          Clone will perform every {{ formattedCloningPeriod }}
        </div>
      </div>
    </div>
    <div :class="$style.buttons">
      <ControlsButton is-white @click="model = false">Cancel</ControlsButton>
      <ControlsButton :is-loading="state.isSubmittingForm" @click="onSubmit()">
        Add
      </ControlsButton>
    </div>
  </ControlsModalsBase>
</template>

<script setup lang="ts">
import { useToast } from "vue-toastification";
import { useRepositoryStore } from "~/store/repository.store";

const model = defineModel<boolean>();
const api = useApi();
const repositoryStore = useRepositoryStore();
const toast = useToast();
const router = useRouter();

const state = reactive({
  form: {
    name: "",
    url: "",
    gitSource: "",
    gitSourceSecretKey: "",
    gitTarget: "",
    gitTargetSecretKey: "",
    cloningPeriod: 1000,
  },
  errors: {
    name: "",
    url: "",
    gitSource: "",
    gitSourceSecretKey: "",
    gitTarget: "",
    gitTargetSecretKey: "",
    cloningPeriod: "",
    submit: "",
  },
  isSubmittingForm: false,
});

const nameModel = computed({
  get: () => state.form.name,
  set: (value: string) => (state.form.name = value),
});

const urlModel = computed({
  get: () => state.form.url,
  set: (value: string) => (state.form.url = value),
});

const gitSourceModel = computed({
  get: () => state.form.gitSource,
  set: (value: string) => (state.form.gitSource = value),
});

const gitSourceSecretKeyModel = computed({
  get: () => state.form.gitSourceSecretKey,
  set: (value: string) => (state.form.gitSourceSecretKey = value),
});

const gitTargetModel = computed({
  get: () => state.form.gitTarget,
  set: (value: string) => (state.form.gitTarget = value),
});

const gitTargetSecretKeyModel = computed({
  get: () => state.form.gitTargetSecretKey,
  set: (value: string) => (state.form.gitTargetSecretKey = value),
});

const cloningPeriodModel = computed({
  get: () => state.form.cloningPeriod,
  set: (value: number) => (state.form.cloningPeriod = value),
});

const formattedCloningPeriod = computed(() => {
  let seconds = state.form.cloningPeriod;

  const years = Math.floor(seconds / 31_536_000);
  seconds %= 31_536_000;

  const days = Math.floor(seconds / 86_400);
  seconds %= 86_400;

  const hours = Math.floor(seconds / 3600);
  seconds %= 3600;

  const minutes = Math.floor(seconds / 60);
  seconds %= 60;

  const parts = [];
  if (years) parts.push(`${years} year${years > 1 ? "s" : ""}`);
  if (days) parts.push(`${days} day${days > 1 ? "s" : ""}`);
  if (hours) parts.push(`${hours} hour${hours > 1 ? "s" : ""}`);
  if (minutes) parts.push(`${minutes} minute${minutes > 1 ? "s" : ""}`);
  if (seconds) parts.push(`${seconds} second${seconds > 1 ? "s" : ""}`);

  return parts.length ? parts.join(" ") : "0 seconds";
});

const onSubmit = async () => {
  try {
    state.isSubmittingForm = true;
    const hasError = validateFields();
    if (hasError) {
      state.isSubmittingForm = false;
      return;
    }

    const res = await repositoryStore.addRepository(api, {
      isEnabled: true,
      name: state.form.name,
      url: state.form.url,
      gitSource: state.form.gitSource,
      gitSourceSecretKey: state.form.gitSourceSecretKey,
      gitTarget: state.form.gitTarget,
      gitTargetSecretKey: state.form.gitTargetSecretKey,
      gitClonePeriodSeconds: state.form.cloningPeriod,
    });

    if (!res?.createdRepository) {
      toast.error("Failed to create Repository");
    }

    await router.push(`/dashboard/repository/${res?.createdRepository.id}`);
  } catch (e: any) {
    toast.error(`Failed to create Repository. Message: ${e?.message}`);
  } finally {
    state.isSubmittingForm = false;
  }
};

const validateFields = () => {
  state.errors.name = "";
  state.errors.url = "";
  state.errors.gitSource = "";
  state.errors.gitSourceSecretKey = "";
  state.errors.gitTarget = "";
  state.errors.gitTargetSecretKey = "";
  state.errors.cloningPeriod = "";
  state.errors.submit = "";

  if (
    state.form.name.trim().length < 3 ||
    state.form.name.trim().length > 200
  ) {
    state.errors.name =
      "Name length should be more than 3 characters and less than 200 characters long";
  }

  if (state.form.url.trim().length >= 512) {
    state.errors.url = "Url should be less than 512 characters long";
  }

  if (
    state.form.gitSource.trim().length < 3 ||
    state.form.gitSource.trim().length >= 512
  ) {
    state.errors.gitSource =
      "git Source should be more than 3 characters and less than 512 characters long";
  }

  if (
    state.form.gitTarget.trim().length < 3 ||
    state.form.gitTarget.trim().length >= 512
  ) {
    state.errors.gitTarget =
      "git Target should be more than 3 characters and less than 512 characters long";
  }

  if (state.form.gitTargetSecretKey.trim().length < 3) {
    state.errors.gitTargetSecretKey =
      "git Target Secret Key should be more than 3 characters long";
  }

  const gitTargetSecretKeyValidation = validateSSHPrivateKey(
    state.form.gitTargetSecretKey,
  );

  const gitSourceSecretKeyValidation = validateSSHPrivateKey(
    state.form.gitSourceSecretKey,
  );

  if (
    state.form.gitSourceSecretKey?.trim().length !== 0 &&
    !gitSourceSecretKeyValidation.result
  ) {
    state.errors.gitSourceSecretKey = gitSourceSecretKeyValidation.message;
  }

  if (!gitTargetSecretKeyValidation.result) {
    state.errors.gitTargetSecretKey = gitTargetSecretKeyValidation.message;
  }

  const hasError = [
    state.errors.name,
    state.errors.url,
    state.errors.gitSource,
    state.errors.gitSourceSecretKey,
    state.errors.gitTarget,
    state.errors.gitTargetSecretKey,
    state.errors.cloningPeriod,
  ].some((val) => val !== "");

  return hasError;
};
</script>

<style lang="scss" module>
.title {
  font-size: 30px;
  margin-bottom: 20px;
}
.content {
  width: 600px;
  margin-bottom: 40px;

  p {
    margin-bottom: 10px;
  }

  .columns {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 20px;

    .cloneDisplay {
      font-size: 12px;
      color: var(--gray-900);
    }
  }
}
.buttons {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
