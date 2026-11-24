<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui';
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useAuth } from '@/composables/useAuth';
import AppLogo from './AppLogo.vue';

const { isAuthenticated, isLoggingOut, logout } = useAuth();

const route = useRoute();
const router = useRouter();

async function signOut() {
  await logout();
  await router.push('/login');
}

const items = computed<NavigationMenuItem[]>(() => [
  {
    label: 'Overview',
    to: '/',
    active: route.path === '/',
  },
  {
    label: 'Packages',
    to: '/packages',
    active: route.path.startsWith('/packages'),
  },
  {
    label: 'Settings',
    to: '/settings',
    active: route.path.startsWith('/settings'),
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
        aria-label="Log out"
        :loading="isLoggingOut"
        @click="signOut"
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
