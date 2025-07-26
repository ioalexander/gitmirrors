import { aggregateApi } from "~/api/aggregate.api";
import { repositoryApi } from "~/api/repository.api";
import { userApi } from "~/api/user.api";

export function createApiClient(config: { baseUrl: string }) {
  return {
    user: userApi({ ...config }),
    repository: repositoryApi({ ...config }),
    aggregate: aggregateApi({ ...config }),
  };
}

export type ApiClient = ReturnType<typeof createApiClient>;
