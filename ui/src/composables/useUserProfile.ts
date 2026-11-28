import { useApi } from '@/composables/useApi';

export interface Profile {
  name: string;
  email: string;
  isAdmin?: boolean;
}

export const useUserProfile = () => {
  return useApi<Profile>('/api/v1/users/me').get().json();
};

export const useUpdateUserProfile = (profile: Profile) => {
  return useApi<Profile>('/api/v1/users/me', { immediate: false }).patch(
    profile,
  );
};
