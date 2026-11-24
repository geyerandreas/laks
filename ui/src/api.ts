import { $fetch } from 'ofetch';

export const api = $fetch.create({
  baseURL: import.meta.env.VITE_API_BASE || 'http://localhost:8080',
  credentials: 'include',
});
