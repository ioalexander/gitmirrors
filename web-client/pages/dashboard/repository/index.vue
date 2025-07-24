<template>
  <Topbar>
    <div :class="$style.topbarContent">
      <h1 :class="$style.title">Repositories</h1>
    </div>
  </Topbar>
  <div :class="$style.container">
    <RepositoryListContainer>
      <RepositoryListAddNew @click="state.isAddNewPopupOpen = true" />
      <RepositoryListItem v-for="item in repositories" :repository="item" />
    </RepositoryListContainer>
  </div>
  <RepositoryAddNewPopup v-model="isAddNewPopupOpen" />
</template>
<script setup lang="ts">
import { useToast } from "vue-toastification";
import { useRepositoryStore } from "~/store/repository.store";

const toast = useToast();
const api = useApi();
const repositoryStore = useRepositoryStore();

const state = reactive({
  isInitialGetAllRequestComplete: false,
  isAddNewPopupOpen: false,
});

const repositories = computed(() => repositoryStore.repositories);
const isAddNewPopupOpen = computed({
  get: () => state.isAddNewPopupOpen,
  set: (val) => (state.isAddNewPopupOpen = val),
});

onMounted(async () => {
  try {
    const res = await repositoryStore.getRepositories(api);

    if (!res || !res?.repositories) {
      toast.error("Failed to fetch repositories");
    }
  } catch (e) {
    console.error(e);
  } finally {
    state.isInitialGetAllRequestComplete = true;
  }
});
</script>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;
}

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
</style>
