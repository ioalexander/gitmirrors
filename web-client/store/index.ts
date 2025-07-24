import { acceptHMRUpdate, defineStore } from "pinia";
import { type NuxtServerInitOptions } from "~/plugins/init.server";
import { useUserStore } from "~/store/user.store";
import { createApiClient } from "~/factories/api.factory";
import axios from "axios";
import { useRepositoryStore } from "./repository.store";
import { useUiStore } from "./ui.store";

export const useStore = defineStore("root", {
  actions: {
    async nuxtServerInit(nuxtApp: any, options: NuxtServerInitOptions) {
      const isServer = !!nuxtApp.ssrContext;

      const config = useRuntimeConfig();
      const api = createApiClient(
        { baseUrl: config.public.serverApiBase },
        {
          serverSideCookiesRaw:
            nuxtApp.ssrContext?.event.req.headers.cookie || "",
        },
      );

      try {
        const baseUrl = isServer
          ? config.public.serverApiBase
          : config.public.apiBase;

        const healthUrl = baseUrl + "/health";
        await axios.get(healthUrl);
      } catch (e) {
        console.error("Server unavailable");
        console.error(e);
        throw createError({
          statusCode: 503,
          statusMessage: "Server unavailable",
        });
      }

      const userStore = useUserStore();
      useRepositoryStore();
      useUiStore();
      await userStore.getMe(api, options);
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}
