import { authApi } from "~/api/auth.api";
import type { NuxtServerInitOptions } from "~/plugins/init.server";

export function createApiClient(
  config: { baseUrl: string },
  options?: NuxtServerInitOptions,
) {
  return {
    auth: authApi({ ...config }, options),
  };
}

export type ApiClient = ReturnType<typeof createApiClient>;
