import { useStore } from "~/store";

export interface NuxtServerInitOptions {
  serverSideCookiesRaw?: string;
}

export default defineNuxtPlugin(async (nuxtApp) => {
  const isServer = !!nuxtApp.ssrContext;

  if (!isServer) {
    console.log("Not a server. Initializing root store canceled.");
    return;
  }

  const store = useStore();

  const event = nuxtApp.ssrContext!.event;

  const serverSideCookiesRaw = event.node.req.headers.cookie;

  await store.nuxtServerInit(nuxtApp, <NuxtServerInitOptions>{
    serverSideCookiesRaw,
  });
});
