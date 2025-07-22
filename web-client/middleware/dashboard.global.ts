import { useUserStore } from "~/store/user.store";

export default defineNuxtRouteMiddleware((to, from) => {
  if (!to.path.startsWith("/dashboard")) return;

  const userStore = useUserStore();

  if (!userStore.user) {
    return navigateTo("/auth/sign-in");
  }
});
