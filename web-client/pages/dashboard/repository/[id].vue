<template>
  <Topbar>
    <h1 :class="$style.topbarTitle">{{ repository?.name }}</h1>
  </Topbar>
  <div :class="$style.grid">
    <RepositoryInfoTab :repository="repository" />
  </div>
</template>
<script setup lang="ts">
import type { Repository } from "~/types/repository";

const route = useRoute();
const api = useApi();
const headers = useRequestHeaders(["cookie"]);
const rawCookies = headers.cookie;

const state = reactive<{ repository: Repository | undefined }>({
  repository: undefined,
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
}

@keyframes fadeInUp {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
