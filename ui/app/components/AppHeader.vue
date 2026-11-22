<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";

const { isAuthenticated, isLoggingOut, logout } = useAuth();

const route = useRoute();

const items = computed<NavigationMenuItem[]>(() => [
  {
    label: "Overview",
    to: "/",
    active: route.path === "/",
  },
  {
    label: "Packages",
    to: "/packages",
    active: route.path.startsWith("/packages"),
  },
  {
    label: "Settings",
    to: "/settings",
    active: route.path.startsWith("/settings"),
  },
]);
</script>

<template>
  <UHeader>
    <template #title>
      <AppLogo />
    </template>

    <template #right>
      <UNavigationMenu v-if="isAuthenticated" :items="items" />

      <UColorModeButton />

      <UButton
        v-if="isAuthenticated"
        color="neutral"
        variant="outline"
        icon="i-lucide-log-out"
        :loading="isLoggingOut"
        @click="logout"
      />
      <UButton
        v-else
        color="primary"
        icon="i-lucide-log-in"
        label="Log in"
        to="/login"
      />
    </template>
  </UHeader>
</template>
