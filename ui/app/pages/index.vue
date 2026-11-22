<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";

const stats = [
  { name: "Total Python Packages", value: "14", icon: "i-lucide-package" },
  { name: "Monthly Downloads", value: "12,482", icon: "i-lucide-download" },
  { name: "Storage Used", value: "412.5 MB", icon: "i-lucide-database" },
];

const columns = [
  { accessorKey: "name", header: "Package Name" },
  { accessorKey: "version", header: "Latest Version" },
  { accessorKey: "uploadedAt", header: "Last Uploaded" },
  { accessorKey: "downloads", header: "Total Downloads" },
  { id: "actions", header: "" },
];

const packages = [
  {
    id: 1,
    name: "data-analyzer-utils",
    version: "2.1.4",
    uploadedAt: "2 hours ago",
    downloads: "1,204",
  },
  {
    id: 2,
    name: "ml-pipeline-core",
    version: "0.8.0-beta",
    uploadedAt: "3 days ago",
    downloads: "450",
  },
  {
    id: 3,
    name: "fastapi-auth-helper",
    version: "1.0.2",
    uploadedAt: "1 week ago",
    downloads: "9,821",
  },
  {
    id: 4,
    name: "pytest-benchmark-logs",
    version: "0.3.1",
    uploadedAt: "3 weeks ago",
    downloads: "1,007",
  },
];

type Package = (typeof packages)[number];

// Dynamic action items for rows
const actionItems = (row: Package) => [
  [
    {
      label: "View Documentation",
      icon: "i-lucide-file-text",
      click: () => console.log("Docs for", row.name),
    },
    {
      label: "Copy pip command",
      icon: "i-lucide-clipboard",
      click: () =>
        navigator.clipboard.writeText(
          `pip install ${row.name}==${row.version}`,
        ),
    },
  ],
  [
    {
      label: "Yank Version",
      icon: "i-lucide-triangle-alert",
      color: "warning",
    },
    { label: "Delete Package", icon: "i-lucide-trash-2", color: "error" },
  ],
];
</script>

<template>
  <!-- Welcome Header -->
  <div>
    <h2 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
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
          <p class="mt-1 text-3xl font-semibold text-gray-900 dark:text-white">
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
    icon="i-heroicons-command-line"
    color="primary"
    variant="soft"
    title="Quick PIP Configuration"
    description="pip install --extra-index-url https://your-domain.local your-package"
    class="my-6"
  />
</template>
