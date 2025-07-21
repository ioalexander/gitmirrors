import { createApiClient } from "~/factories/api.factory";

export const useApi = () => {
  const config = useRuntimeConfig();
  const nuxtApp = useNuxtApp();
  const isServer = !!nuxtApp.ssrContext;

  const baseUrl = isServer
    ? config.public.serverApiBase
    : config.public.apiBase;

  return createApiClient({ baseUrl });
};

export type Api = ReturnType<typeof useApi>;
