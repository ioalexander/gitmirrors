import { useUserStore } from "~/store/user.store";

export default defineNuxtRouteMiddleware((to, from) => {
  if (!to.path.startsWith("/dashboard")) return;

  const userStore = useUserStore();

  return;

  if (!userStore.user) {
    return navigateTo("/auth/sign-in");
  }
});
