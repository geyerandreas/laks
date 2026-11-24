<script setup lang="ts">
import { reactive } from 'vue';
const profile = reactive({ name: '', email: '' });
const password = reactive({ current: '', next: '', confirmation: '' });
</script>

<template>
  <div class="space-y-6">
    <div>
      <h2 class="text-2xl font-bold tracking-tight text-highlighted">
        General
      </h2>
      <p class="text-muted">Manage your account details and password.</p>
    </div>

    <UAlert
      color="warning"
      variant="subtle"
      icon="i-lucide-info"
      title="Account settings are not connected yet"
      description="Your current details cannot be loaded, and changes cannot be saved or your account deleted yet."
    />

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
        <UButton label="Save changes" disabled />
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
  </div>
</template>
