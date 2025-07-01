import { acceptHMRUpdate, defineStore } from "pinia";
import { type NuxtServerInitOptions } from "~/plugins/init.server";
import { useAuthStore } from "~/store/auth.store";
import { createApiClient } from "~/factories/api.factory";
import axios from "axios";

export const useStore = defineStore("root", {
  actions: {
    async nuxtServerInit(nuxtApp: any, options: NuxtServerInitOptions) {
      const config = useRuntimeConfig();
      const api = createApiClient(
        { baseUrl: config.public.apiBase },
        {
          serverSideCookiesRaw:
            nuxtApp.ssrContext?.event.req.headers.cookie || "",
        },
      );

      try {
        const healthUrl = config.public.apiBase + "/health";
        console.error(healthUrl);
        await axios.get(healthUrl);
      } catch (e) {
        console.error("Server unavailable");
        console.error(e);
        throw createError({
          statusCode: 503,
          statusMessage: "Server unavailable",
        });
      }

      const authStore = useAuthStore();
      await authStore.getMe(api);
    },
  },
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot));
}
