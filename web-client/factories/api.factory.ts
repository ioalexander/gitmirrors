import { aggregateApi } from "~/api/aggregate.api";
import { repositoryApi } from "~/api/repository.api";
import { userApi } from "~/api/user.api";
import type { NuxtServerInitOptions } from "~/plugins/init.server";

export function createApiClient(
  config: { baseUrl: string },
  options?: NuxtServerInitOptions,
) {
  return {
    user: userApi({ ...config }, options),
    repository: repositoryApi({ ...config }),
    aggregate: aggregateApi({ ...config }),
  };
}

export type ApiClient = ReturnType<typeof createApiClient>;
