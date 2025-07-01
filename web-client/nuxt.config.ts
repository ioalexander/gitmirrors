// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-05-15",
  devtools: { enabled: process.env.NODE_ENV === "development" },
  debug: process.env.NODE_ENV === "development",
  nitro: {
    logLevel: "debug",
  },

  css: ["assets/scss/main.scss"],

  runtimeConfig: {
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_URL,
    },
  },

  modules: [
    "@nuxt/eslint",
    [
      "@pinia/nuxt",
      {
        autoImports: ["defineStore", "acceptHMRUpdate"],
      },
    ],
    "@nuxt/fonts",
    "@nuxt/icon",
    "@nuxt/image",
  ],

  postcss: {
    plugins: {
      cssnano: {
        preset: [
          "default",
          {
            minifyFontValues: { removeQuotes: false },
            cssDeclarrationSorter: false,
          },
        ],
      },
    },
  },
});
