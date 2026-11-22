<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const links = [
  [
    {
      label: "General",
      icon: "i-lucide-user",
      to: "/settings/general",
      exact: true,
    },
    {
      label: "Users",
      icon: "i-lucide-users",
      to: "/settings/users",
    },
  ],
] satisfies NavigationMenuItem[][];
</script>

<template>
  <UNavigationMenu :items="links" variant="pill" highlight class="-mt-8" />

  <USeparator class="-mt-px pb-4" />

  <RouterView />
</template>
