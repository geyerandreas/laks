<script setup lang="ts">
import * as z from 'zod';
import type { FormSubmitEvent, AuthFormField } from '@nuxt/ui';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { api } from '@/api';

const router = useRouter();
const isSubmitting = ref(false);
const errorMessage = ref('');

const fields: AuthFormField[] = [
  {
    name: 'email',
    type: 'email',
    label: 'Email',
    placeholder: 'Enter your email',
    required: true,
  },
  {
    name: 'password',
    label: 'Password',
    type: 'password',
    placeholder: 'Enter your password',
    required: true,
  },
  {
    name: 'remember',
    label: 'Remember me',
    type: 'checkbox',
  },
];

// TODO
// const providers = [{
//   label: 'Google',
//   onClick: () => {
//     toast.add({ title: 'Google', description: 'Login with Google' })
//   }
// }, {
//   label: 'GitHub',
//   onClick: () => {
//     toast.add({ title: 'GitHub', description: 'Login with GitHub' })
//   }
// }]

const schema = z.object({
  email: z.email('Invalid email'),
  password: z.string('Password is required'),
});

type Schema = z.output<typeof schema>;

async function onSubmit(payload: FormSubmitEvent<Schema>) {
  errorMessage.value = '';
  isSubmitting.value = true;

  try {
    await api('/api/v1/auth/login', {
      method: 'POST',
      credentials: 'include',
      body: payload.data,
    });

    await router.push('/');
  } catch (error: unknown) {
    const status =
      typeof error === 'object' && error !== null && 'response' in error
        ? (error as { response?: { status?: number } }).response?.status
        : undefined;

    errorMessage.value =
      status === 401
        ? 'The email or password is incorrect.'
        : 'Unable to sign in right now. Please try again.';
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen bg-elevated/20">
    <main
      class="mx-auto flex min-h-[calc(100vh-4rem)] w-full max-w-6xl items-center justify-center px-4 py-12 sm:px-6 lg:px-8"
    >
      <UPageCard class="w-full max-w-md shadow-xl shadow-primary/5">
        <UAuthForm
          :schema="schema"
          title="Welcome back"
          description="Sign in to manage your private Python packages."
          icon="i-lucide-lock"
          :fields="fields"
          :loading="isSubmitting"
          :submit="{ label: 'Sign in', block: true }"
          @submit="onSubmit"
        >
          <template #validation>
            <UAlert
              v-if="errorMessage"
              color="error"
              variant="subtle"
              :description="errorMessage"
              icon="i-lucide-circle-alert"
            />
          </template>
          <template #footer>
            <p class="text-center text-sm text-muted">
              Don't have an account?
              <ULink
                to="/register"
                class="font-medium text-primary hover:underline"
              >
                Create one
              </ULink>
            </p>
          </template>
        </UAuthForm>
      </UPageCard>
    </main>
  </div>
</template>
