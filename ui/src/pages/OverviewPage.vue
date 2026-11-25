<script setup lang="ts">
const stats = [
  { name: 'Total Python Packages', value: '14', icon: 'i-lucide-package' },
  { name: 'Monthly Downloads', value: '12,482', icon: 'i-lucide-download' },
  { name: 'Storage Used', value: '412.5 MB', icon: 'i-lucide-database' },
];
</script>

<template>
  <div>
    <!-- Welcome Header -->
    <div>
      <h2
        class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white"
      >
        Welcome back, developer!
      </h2>
      <p class="text-gray-500 dark:text-gray-400">
        Manage your private Python repository and deployments.
      </p>
    </div>

    <!-- Quick Metrics Grid -->
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-3 my-4">
      <UCard v-for="stat in stats" :key="stat.name" class="overflow-hidden">
        <div class="flex items-center justify-between">
          <div>
            <p
              class="text-sm font-medium text-gray-500 truncate dark:text-gray-400"
            >
              {{ stat.name }}
            </p>
            <p
              class="mt-1 text-3xl font-semibold text-gray-900 dark:text-white"
            >
              {{ stat.value }}
            </p>
          </div>
          <div class="p-3 bg-primary-50 dark:bg-primary-950/50 rounded-lg">
            <UIcon :name="stat.icon" class="w-6 h-6 text-primary-500" />
          </div>
        </div>
      </UCard>
    </div>

    <!-- Quick Twine Instructions -->
    <UAlert
      icon="i-lucide-terminal"
      color="primary"
      variant="soft"
      title="Quick PIP Configuration"
      description="pip install --extra-index-url https://your-domain.local your-package"
      class="my-6"
    />
  </div>
</template>
