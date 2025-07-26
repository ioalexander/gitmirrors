<template>
  <div>
    <Topbar>
      <div :class="$style.topbarContent">
        <h1 :class="$style.title">{{ repository?.name }}</h1>
        <div :class="$style.right">
          <ControlsButton is-red @click="state.isDeleteConfirmModalOpen = true"
            >Delete</ControlsButton
          >
        </div>
      </div>
    </Topbar>
    <ControlsModalsConfirm
      v-model="isDeleteConfirmModalOpenModel"
      is-red
      @cancel="state.isDeleteConfirmModalOpen = false"
      @confirm="deleteRepository"
    >
      <template #title>Delete "{{ repository?.name }}"?</template>
      <template #content
        >Are you sure you want to delete repository "{{ repository?.name }}"?
        <br >
        You can't undo this.</template
      >
    </ControlsModalsConfirm>
    <div :class="$style.grid">
      <RepositoryInfoTab :repository="repository" />
      <RepositoryCooldownTab
        :repository="repository"
        :clone-due="state.cloneDue"
      />
      <RepositoryLogsTab :logs="repositoryLogs" />
    </div>
  </div>
</template>
<script setup lang="ts">
import { useToast } from "vue-toastification";
import { useRepositoryStore } from "~/store/repository.store";
import type { Repository } from "~/types/repository";
import type { RepositoryLog } from "~/types/repositoryLog";

const route = useRoute();
const repositoryStore = useRepositoryStore();
const api = useApi();
const headers = useRequestHeaders(["cookie"]);
const toast = useToast();
const rawCookies = headers.cookie;
const router = useRouter();

let cloneDueUpdateIntervalId: number | undefined;
let fetchRepositoryIntervalId: number | undefined;
let fetchRepositoryLogsIntervalId: number | undefined;

const state = reactive<{
  repository: Repository | undefined;
  repositoryLogs: RepositoryLog[];
  cloneDue: number | undefined;
  cloneDueExpired: boolean;
  isDeleteConfirmModalOpen: boolean;
}>({
  repository: undefined,
  repositoryLogs: [],
  cloneDue: undefined,
  cloneDueExpired: false,
  isDeleteConfirmModalOpen: false,
});

const isDeleteConfirmModalOpenModel = computed({
  get: () => state.isDeleteConfirmModalOpen,
  set: (val) => (state.isDeleteConfirmModalOpen = val),
});

const repository = computed(() => state.repository);
const repositoryLogs = computed(() => state.repositoryLogs);

const fetchRepository = async (id: string) => {
  try {
    const res = await api.repository.getRepository(id, {
      serverSideCookiesRaw: rawCookies,
    });
    state.repository = res?.data?.repository;

    if (!state.repository) {
      throw createError({
        statusCode: 404,
        statusMessage: "Repository not found",
      });
    }
  } catch (e: unknown) {
    const message = e instanceof Error ? e.message : String(e);
    throw createError({
      statusCode: 500,
      statusMessage: `Failed to fetch repository. Error: ${message}`,
    });
  }
};

const fetchRepositoryLogs = async (id: string) => {
  try {
    const res = await api.repository.getRepositoryLogs(id, {
      serverSideCookiesRaw: rawCookies,
    });

    if (!res.data.repositoryLogs) {
      throw createError({
        statusCode: 404,
        statusMessage: "Repository logs not found",
      });
    }

    state.repositoryLogs = res?.data?.repositoryLogs
      ?.slice()
      .sort((a, b) => {
        const dateA = new Date(a.createdAt).getTime();
        const dateB = new Date(b.createdAt).getTime();
        return dateB - dateA;
      })
      .slice(0, 10);
  } catch (e: unknown) {
    const message = e instanceof Error ? e.message : String(e);

    throw createError({
      statusCode: 500,
      statusMessage: `Failed to fetch repository logs. Error: ${message}`,
    });
  }
};

const updateCloneDue = () => {
  if (
    !!state.repository?.lastCloneAt &&
    !!state.repository?.gitClonePeriodSeconds
  ) {
    state.cloneDue = calculateCloneDue(
      new Date(state.repository.lastCloneAt),
      state.repository?.gitClonePeriodSeconds,
    );

    state.cloneDueExpired = state.cloneDue === 0;
  }
};

const calculateCloneDue = (lastCloneAt: Date, period: number): number => {
  const lastTime = lastCloneAt.getTime();

  const now = Date.now();
  const elapsedSeconds = (now - lastTime) / 1000;
  const remaining = period - elapsedSeconds;
  return remaining > 0 ? remaining : 0;
};

const fetchRepositoryInterval = async () => {
  if (state.cloneDueExpired) {
    try {
      await fetchRepository(route.params.id as string);
    } catch {
      toast.error("Failed to fetch repository");
    }
  }
};

const fetchRepositoryLogsInterval = async () => {
  if (state.cloneDueExpired) {
    try {
      await fetchRepositoryLogs(route.params.id as string);
    } catch {
      toast.error("Failed to fetch repository logs");
    }
  }
};

const deleteRepository = async () => {
  const res = await repositoryStore.deleteRepository(
    api,
    repository.value?.id as string,
  );

  if (!res?.deletedRepository) {
    toast.error("Failed to delete repository!");
    return;
  }

  toast.success("Repository deleted succesfully!");

  await router.push("/dashboard/repository");
};

onMounted(() => {
  updateCloneDue();
  cloneDueUpdateIntervalId = window.setInterval(updateCloneDue, 200);
  fetchRepositoryIntervalId = window.setInterval(
    fetchRepositoryInterval,
    10000,
  );
  fetchRepositoryLogsIntervalId = window.setInterval(
    fetchRepositoryLogsInterval,
    2000,
  );
});

onUnmounted(() => {
  if (cloneDueUpdateIntervalId) clearInterval(cloneDueUpdateIntervalId);
  if (fetchRepositoryIntervalId) clearInterval(fetchRepositoryIntervalId);
  if (fetchRepositoryLogsIntervalId)
    clearInterval(fetchRepositoryLogsIntervalId);
});

await fetchRepository(route.params.id as string);
await fetchRepositoryLogs(route.params.id as string);
</script>
<style lang="scss" module>
.topbarContent {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  .title {
    font-size: 20px;
  }
  .right {
    display: flex;
  }
}

.grid {
  width: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  grid-gap: 20px;

  > * {
    opacity: 0;
    transform: translateY(10px);
    animation: fadeInUp 0.5s forwards;
  }

  @for $i from 1 through 10 {
    > *:nth-child(#{$i}) {
      animation-delay: #{($i - 1) * 0.05}s;
    }
  }

  @media all and (max-width: 1500px) {
    & {
      grid-template-columns: 1fr 1fr;
    }
  }

  @media all and (max-width: 900px) {
    & {
      grid-template-columns: 1fr;
    }
  }
}

@keyframes fadeInUp {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
