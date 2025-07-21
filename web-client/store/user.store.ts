import { useApi, type Api } from "~/composables/useApi";
import type { NuxtServerInitOptions } from "~/plugins/init.server";

interface State {
  user?: { id: string; createdAt: Date; updatedAt: Date };
  session?: { id: string; createdAt: Date; updatedAt: Date };
  authState: {
    screen: string;
    email: string;
  };
}

export const useUserStore = defineStore("user", {
  state: (): State => ({
    user: undefined,
    session: undefined,
    authState: {
      screen: "welcome",
      email: "",
    },
  }),
  getters: {
    isAuthenticated: (state) => !!state.user && !!state.session,
  },
  actions: {
    async logout() {
      const api = useApi();

      try {
        await api.user.logout();
      } catch (e: any) {
        console.error("Failed to perform logout. Error: ", e);
        this.UNSET_USER();
      }

      this.UNSET_USER();
    },
    async getMe(api: Api, nuxtServerInitOptions?: NuxtServerInitOptions) {
      try {
        const meResponse = await api.user.getMe();

        if (meResponse?.message === "Unauthorized") {
          throw new Error("User not authorized. Bad token");
        }

        const user = meResponse?.data?.user;
        const session = meResponse?.data?.session;

        if (!user) {
          throw new Error("meResponse.data.user is undefined");
        }

        if (!session) {
          throw new Error("meResponse.data.session is undefined");
        }

        this.SET_USER({
          id: user.id,
          updatedAt: new Date(user.updatedAt),
          createdAt: new Date(user.createdAt),
        });
        this.SET_SESSION({
          id: session.id,
          updatedAt: new Date(session.updatedAt),
          createdAt: new Date(session.createdAt),
        });

        return { user, session };
      } catch (e: any) {
        this.UNSET_USER();
        this.UNSET_SESSION();
      }
    },
    SET_USER(user: { id: string; createdAt: Date; updatedAt: Date }) {
      this.user = { ...user };
    },
    UNSET_USER() {
      this.user = undefined;
    },
    SET_SESSION(session: { id: string; createdAt: Date; updatedAt: Date }) {
      this.session = { ...session };
    },
    UNSET_SESSION() {
      this.session = undefined;
    },
    SET_AUTH_STATE(state: { screen: string; email: string }) {
      this.authState = { ...state };
    },
    RESET_AUTH_STATE() {
      this.authState = { screen: "welcome", email: "" };
    },
  },
});
