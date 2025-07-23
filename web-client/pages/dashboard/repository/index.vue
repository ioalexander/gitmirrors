<template>
  <div :class="$style.container">
    <RepositoryListContainer>
      <RepositoryListItem v-for="item in repositories" :repository="item" />
      <RepositoryListAddNew @click="state.isAddNewPopupOpen = true" />
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
  }
});
</script>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;
}
</style>
