import { createFetch } from '@vueuse/core';

export const useApi = createFetch({
  baseUrl: import.meta.env.VITE_API_BASE || 'http://localhost:8080',
  fetchOptions: {
    credentials: 'include',
  },
});
