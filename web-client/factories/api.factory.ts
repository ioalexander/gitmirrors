import { userApi } from "~/api/user.api";
import type { NuxtServerInitOptions } from "~/plugins/init.server";

export function createApiClient(
  config: { baseUrl: string },
  options?: NuxtServerInitOptions,
) {
  return {
    user: userApi({ ...config }, options),
  };
}

export type ApiClient = ReturnType<typeof createApiClient>;
