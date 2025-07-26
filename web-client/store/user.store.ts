import { useApi, type Api } from "~/composables/useApi";
import type { NuxtServerInitOptions } from "~/plugins/init.server";

interface State {
  user?: { id: string; username: string; createdAt: Date; updatedAt: Date };
  authState: {
    screen: string;
    email: string;
  };
}

export const useUserStore = defineStore("user", {
  state: (): State => ({
    user: undefined,
    authState: {
      screen: "welcome",
      email: "",
    },
  }),
  getters: {
    isAuthenticated: (state) => !!state.user,
  },
  actions: {
    async logout() {
      const api = useApi();

      try {
        await api.user.logout();
      } catch (e: unknown) {
        if (e instanceof Error) {
          console.error("Failed to perform logout. Error:", e.message);
        } else {
          console.error("Failed to perform logout. Unknown error:", e);
        }
        this.UNSET_USER();
      }

      this.UNSET_USER();
    },
    async getMe(api: Api, nuxtServerInitOptions?: NuxtServerInitOptions) {
      try {
        console.log("getting user...");
        const meResponse = await api.user.getMe(nuxtServerInitOptions);
        console.log("got user: ", meResponse);

        if (meResponse?.message === "Unauthorized") {
          throw new Error("User not authorized. Bad token");
        }

        const user = meResponse?.data?.user;

        if (!user) {
          throw new Error("meResponse.data.user is undefined");
        }

        this.SET_USER({
          id: user.id,
          username: user.username,
          updatedAt: new Date(user.updatedAt),
          createdAt: new Date(user.createdAt),
        });

        return { user };
      } catch (e: unknown) {
        if (e instanceof Error) {
          console.error("get user error:", e.message);
        } else {
          console.error("get user error (unknown):", e);
        }
        this.UNSET_USER();
      }
    },
    SET_USER(user: {
      id: string;
      username: string;
      createdAt: Date;
      updatedAt: Date;
    }) {
      this.user = { ...user };
    },
    UNSET_USER() {
      this.user = undefined;
    },
    SET_AUTH_STATE(state: { screen: string; email: string }) {
      this.authState = { ...state };
    },
    RESET_AUTH_STATE() {
      this.authState = { screen: "welcome", email: "" };
    },
  },
});
