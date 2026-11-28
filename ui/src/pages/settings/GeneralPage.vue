<script setup lang="ts">
import {
  useUserProfile,
  useUpdateUserProfile,
  type Profile,
} from '@/composables/useUserProfile';
import { reactive, watch } from 'vue';

const toast = useToast();

const profile = reactive<Profile>({ name: '', email: '' });

const password = reactive({ current: '', next: '', confirmation: '' });

const { data, isFetching, error } = useUserProfile();

watch(
  data,
  (loadedProfile) => {
    if (loadedProfile) {
      Object.assign(profile, loadedProfile);
    }
  },
  { immediate: true },
);

const { execute: updateProfile, isFetching: updateProfilePending } =
  useUpdateUserProfile(profile);

const saveProfile = async () => {
  try {
    await updateProfile(true);
    toast.add({
      title: 'Profile updated',
      description: 'Your profile has been successfully updated.',
      color: 'success',
    });
  } catch {
    toast.add({
      title: 'Failed to update profile',
      description: 'An error occurred while updating your profile.',
      color: 'error',
    });
  }
};
</script>

<template>
  <div class="space-y-6">
    <UEmpty
      v-if="isFetching"
      icon="i-lucide-file"
      loading
      loading-icon="i-lucide-loader"
      title="Loading profile"
    />

    <UAlert
      v-if="error || (!isFetching && !data)"
      color="error"
      variant="subtle"
      icon="i-lucide-alert-circle"
      title="Failed to load profile"
      description="Your current details cannot be loaded, and changes cannot be saved or your account deleted yet."
    />

    <template v-if="data && !isFetching && !error">
      <div>
        <h2 class="text-2xl font-bold tracking-tight text-highlighted">
          General
        </h2>
        <p class="text-muted">Manage your account details and password.</p>
      </div>

      <UCard>
        <template #header>
          <h3 class="font-semibold text-highlighted">Profile</h3>
          <p class="text-sm text-muted">Update your name and email address.</p>
        </template>

        <div class="space-y-4">
          <UFormField label="Name" name="name">
            <UInput
              v-model="profile.name"
              class="w-full"
              autocomplete="name"
              placeholder="Your name"
            />
          </UFormField>
          <UFormField label="Email address" name="email">
            <UInput
              v-model="profile.email"
              class="w-full"
              type="email"
              autocomplete="email"
              placeholder="you@example.com"
            />
          </UFormField>
          <UButton
            label="Save changes"
            @click="saveProfile"
            :loading="updateProfilePending"
          />
        </div>
      </UCard>

      <UCard>
        <template #header>
          <h3 class="font-semibold text-highlighted">Password</h3>
          <p class="text-sm text-muted">Change your account password.</p>
        </template>

        <div class="space-y-4">
          <UFormField label="Current password" name="current">
            <UInput
              v-model="password.current"
              class="w-full"
              type="password"
              autocomplete="current-password"
            />
          </UFormField>
          <UFormField label="New password" name="next">
            <UInput
              v-model="password.next"
              class="w-full"
              type="password"
              autocomplete="new-password"
            />
          </UFormField>
          <UFormField label="Confirm new password" name="confirmation">
            <UInput
              v-model="password.confirmation"
              class="w-full"
              type="password"
              autocomplete="new-password"
            />
          </UFormField>
          <UButton label="Update password" disabled />
        </div>
      </UCard>

      <UCard>
        <template #header>
          <h3 class="font-semibold text-highlighted">Delete account</h3>
          <p class="text-sm text-muted">
            Permanently delete your account and its access to the repository.
          </p>
        </template>

        <UButton label="Delete account" color="error" variant="soft" disabled />
      </UCard>
    </template>
  </div>
</template>
