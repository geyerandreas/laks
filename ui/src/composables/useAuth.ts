import { ref } from 'vue';
import { api } from '@/api';

const isAuthenticated = ref<boolean | null>(null);

export async function refreshAuthStatus() {
  try {
    await api('/api/v1/auth/status');
    isAuthenticated.value = true;
  } catch {
    isAuthenticated.value = false;
  }
  return isAuthenticated.value;
}

export function useAuth() {
  const isLoggingOut = ref(false);

  async function logout() {
    isLoggingOut.value = true;
    try {
      await api('/api/v1/auth/logout', { method: 'POST' });
      isAuthenticated.value = false;
    } finally {
      isLoggingOut.value = false;
    }
  }

  return { isAuthenticated, isLoggingOut, refreshAuthStatus, logout };
}
