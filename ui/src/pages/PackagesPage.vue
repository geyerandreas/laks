<script setup lang="ts">
import type { TableColumn } from '@nuxt/ui';

const columns = [
  { accessorKey: 'name', header: 'Package Name' },
  { accessorKey: 'version', header: 'Latest Version' },
  { accessorKey: 'uploadedAt', header: 'Last Uploaded' },
  { accessorKey: 'downloads', header: 'Total Downloads' },
  { id: 'actions', header: '' },
];

const packages = [
  {
    id: 1,
    name: 'data-analyzer-utils',
    version: '2.1.4',
    uploadedAt: '2 hours ago',
    downloads: '1,204',
  },
  {
    id: 2,
    name: 'ml-pipeline-core',
    version: '0.8.0-beta',
    uploadedAt: '3 days ago',
    downloads: '450',
  },
  {
    id: 3,
    name: 'fastapi-auth-helper',
    version: '1.0.2',
    uploadedAt: '1 week ago',
    downloads: '9,821',
  },
  {
    id: 4,
    name: 'pytest-benchmark-logs',
    version: '0.3.1',
    uploadedAt: '3 weeks ago',
    downloads: '1,007',
  },
];

type Package = (typeof packages)[number];

// Dynamic action items for rows
const actionItems = (row: Package) => [
  [
    {
      label: 'View Documentation',
      icon: 'i-lucide-file-text',
      onSelect: () => console.log('Docs for', row.name),
    },
    {
      label: 'Copy pip command',
      icon: 'i-lucide-clipboard',
      onSelect: () =>
        navigator.clipboard.writeText(
          `pip install ${row.name}==${row.version}`,
        ),
    },
  ],
  [
    {
      label: 'Yank Version',
      icon: 'i-lucide-triangle-alert',
      color: 'warning',
    },
    { label: 'Delete Package', icon: 'i-lucide-trash-2', color: 'error' },
  ],
];
</script>

<template>
  <div class="space-y-6">
    <div>
      <h2
        class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white"
      >
        Your Uploaded Packages
      </h2>
      <p class="text-gray-500 dark:text-gray-400">
        A list of all the Python packages you've uploaded to your private
        repository, along with their latest versions, upload dates, and download
        counts.
      </p>
    </div>

    <UCard>
      <UTable :data="packages" :columns="columns as TableColumn<Package>[]">
        <!-- Custom Template for Name Column -->
        <template #name-cell="{ row }">
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-package" class="w-5 h-5 text-gray-400" />
            <span class="font-medium text-gray-900 dark:text-white">{{
              row.original.name
            }}</span>
          </div>
        </template>

        <!-- Custom Template for Latest Version Badge -->
        <template #version-cell="{ row }">
          <UBadge size="xs" variant="subtle" color="primary">
            {{ row.original.version }}
          </UBadge>
        </template>

        <!-- Action Buttons for individual packages -->
        <template #actions-cell="{ row }">
          <UDropdownMenu :items="actionItems(row.original)">
            <UButton color="neutral" variant="ghost" icon="i-lucide-ellipsis" />
          </UDropdownMenu>
        </template>
      </UTable>
    </UCard>
  </div>
</template>
