<script setup lang="ts">
import { computed } from "vue";
import { useColorMode } from "@vueuse/core";

const colorMode = useColorMode();
const themeColor = computed(() =>
  colorMode.value === "dark" ? "#101114" : "#ffffff",
);

useHead({
  link: [{ rel: "icon", type: "image/svg+xml", href: "/favicon.svg" }],
  meta: [{ name: "theme-color", content: themeColor }],
  titleTemplate: (title) =>
    title
      ? `${title} - Laks - Python Package Registry `
      : "Laks - Python Package Registry",
});
</script>

<template>
  <UApp>
    <AppHeader />

    <UMain>
      <NuxtLayout>
        <UContainer class="py-8">
          <NuxtPage />
        </UContainer>
      </NuxtLayout>
    </UMain>

    <UFooter />
  </UApp>
</template>
