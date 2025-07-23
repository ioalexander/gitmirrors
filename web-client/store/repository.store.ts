import { useApi, type Api } from "~/composables/useApi";
import type { Repository } from "~/types/repository";

interface State {
  repositories: Repository[];
}

export const useRepositoryStore = defineStore("repository", {
  state: (): State => ({
    repositories: [],
  }),
  actions: {
    async getRepositories(api: Api) {
      try {
        const res = await api.repository.getRepositories();

        const repositories = res?.data?.repositories;

        this.SET_REPOSITORIES(repositories);

        return { repositories };
      } catch (e: any) {
        this.UNSET_REPOSITORIES();
      }
    },
    async addRepository(
      api: Api,
      data: Omit<Repository, "id" | "createdAt" | "updatedAt" | "lastCloneAt">,
    ) {
      try {
        const res = await api.repository.addRepository(data);
        const createdRepository = res?.data.createdRepository;

        this.repositories = [...this.repositories, createdRepository];

        return { createdRepository };
      } catch (e: any) {
        console.error(e);
      }
    },
    SET_REPOSITORIES(repositories: Repository[]) {
      this.repositories = repositories;
    },
    UNSET_REPOSITORIES() {
      this.repositories = [];
    },
  },
});
