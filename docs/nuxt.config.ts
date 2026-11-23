export default defineNuxtConfig({
  modules: ["@nuxt/eslint"],

  extends: ["docus"],

  site: {
    name: "laks: The self-hosted Python registry",
  },

  colorMode: {
    preference: "dark",
    fallback: "dark",
  },
});
