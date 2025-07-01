import { createApiClient } from "~/factories/api.factory";

export const useApi = () => {
  const config = useRuntimeConfig();
  return createApiClient({ baseUrl: config.public.apiBase });
};

export type Api = ReturnType<typeof useApi>;
