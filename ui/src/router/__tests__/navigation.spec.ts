import { beforeEach, describe, expect, it, vi } from 'vitest';

const api = vi.hoisted(() =>
  vi.fn<(path: string, options?: { method: string }) => Promise<object>>(),
);
vi.mock('@/api', () => ({ api }));

import router from '../index';
import { useAuth } from '@/composables/useAuth';

describe('authentication navigation', () => {
  beforeEach(() => {
    api.mockReset();
  });

  it('redirects unauthenticated users to login and permits registration', async () => {
    api.mockRejectedValue(new Error('unauthorized'));
    await router.push('/settings/users');
    expect(router.currentRoute.value.path).toBe('/login');
    expect(useAuth().isAuthenticated.value).toBe(false);
    await router.push('/register');
    expect(router.currentRoute.value.path).toBe('/register');
  });

  it('redirects authenticated users away from login and resolves settings and dashboard', async () => {
    api.mockResolvedValue({});
    await router.push('/login');
    expect(router.currentRoute.value.path).toBe('/');
    expect(useAuth().isAuthenticated.value).toBe(true);
    await router.push('/settings');
    expect(router.currentRoute.value.path).toBe('/settings/general');
    await router.push('/dashboard');
    expect(router.currentRoute.value.path).toBe('/');
  });

  it('clears authentication after logout', async () => {
    api.mockResolvedValue({});
    const auth = useAuth();
    await auth.refreshAuthStatus();
    await auth.logout();
    expect(api).toHaveBeenCalledWith('/api/v1/auth/logout', { method: 'POST' });
    expect(auth.isAuthenticated.value).toBe(false);
    expect(auth.isLoggingOut.value).toBe(false);
  });
});
