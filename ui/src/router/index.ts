import { createRouter, createWebHistory } from 'vue-router';
import { refreshAuthStatus } from '@/composables/useAuth';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: () => import('@/pages/OverviewPage.vue') },
    { path: '/dashboard', redirect: '/' },
    {
      path: '/login',
      component: () => import('@/pages/LoginPage.vue'),
      meta: { public: true },
    },
    {
      path: '/register',
      component: () => import('@/pages/RegisterPage.vue'),
      meta: { public: true },
    },
    { path: '/packages', component: () => import('@/pages/PackagesPage.vue') },
    {
      path: '/settings',
      component: () => import('@/pages/SettingsPage.vue'),
      children: [
        { path: '', redirect: '/settings/general' },
        {
          path: 'general',
          component: () => import('@/pages/settings/GeneralPage.vue'),
        },
        {
          path: 'users',
          component: () => import('@/pages/settings/UsersPage.vue'),
        },
      ],
    },
  ],
});

router.beforeEach(async (to) => {
  const authenticated = await refreshAuthStatus();
  if (!authenticated && !to.meta.public) return '/login';
  if (authenticated && to.meta.public) return '/';
});

export default router;
