<template>
  <Topbar>
    <h1 :class="$style.topbarTitle">{{ repository?.name }}</h1>
  </Topbar>
  <div :class="$style.grid">
    <RepositoryInfoTab :repository="repository" />
    <RepositoryCooldownTab
      :repository="repository"
      :clone-due="state.cloneDue"
    />
  </div>
</template>
<script setup lang="ts">
import { useToast } from "vue-toastification";
import type { Repository } from "~/types/repository";

const route = useRoute();
const api = useApi();
const headers = useRequestHeaders(["cookie"]);
const toast = useToast();
const rawCookies = headers.cookie;

let cloneDueUpdateIntervalId: number | undefined;
let fetchRepositoryIntervalId: number | undefined;

const state = reactive<{
  repository: Repository | undefined;
  cloneDue: number | undefined;
  cloneDueExpired: boolean;
}>({
  repository: undefined,
  cloneDue: undefined,
  cloneDueExpired: false,
});

const repository = computed(() => state.repository);

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
  } catch (e: any) {
    throw createError({
      statusCode: 500,
      statusMessage: `Failed to fetch repository. Error: ${e?.message}`,
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
      toast.success("Repository data updated!");
    } catch {
      toast.error("Failed to fetch repository");
    }
  }
};

onMounted(() => {
  updateCloneDue();
  cloneDueUpdateIntervalId = window.setInterval(updateCloneDue, 200);
  fetchRepositoryIntervalId = window.setInterval(fetchRepositoryInterval, 1000);
});

onUnmounted(() => {
  if (cloneDueUpdateIntervalId) clearInterval(cloneDueUpdateIntervalId);
  if (fetchRepositoryIntervalId) clearInterval(fetchRepositoryIntervalId);
});

await fetchRepository(route.params.id as string);
</script>
<style lang="scss" module>
.topbarTitle {
  font-size: 20px;
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
