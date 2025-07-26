import type { Api } from "~/composables/useApi";
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
      } catch (e: unknown) {
        if (e instanceof Error) {
          console.error("get repositories error:", e.message);
        } else {
          console.error("get repositories error (unknown):", e);
        }
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
      } catch (e: unknown) {
        if (e instanceof Error) {
          console.error("add repository error:", e.message);
        } else {
          console.error("add repository error (unknown):", e);
        }
      }
    },
    async deleteRepository(api: Api, id: string) {
      try {
        const res = await api.repository.deleteRepository(id);
        const deletedRepository = res?.data.repository;

        this.repositories = this.repositories.filter(
          (r) => r.id !== deletedRepository?.id,
        );

        return { deletedRepository };
      } catch (e: unknown) {
        if (e instanceof Error) {
          console.error("delete repository error:", e.message);
        } else {
          console.error("delete repository error (unknown):", e);
        }
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
