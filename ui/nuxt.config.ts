// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@nuxt/eslint", "@nuxt/ui"],

  devtools: {
    enabled: true,
  },

  css: ["~/assets/css/main.css"],

  colorMode: {
    preference: "dark",
    fallback: "dark",
  },

  runtimeConfig: {
    public: {
      apiBase: "http://localhost:8080",
    },
  },

  routeRules: {
    "/": { prerender: true },
  },

  compatibilityDate: "2026-06-30",
});
