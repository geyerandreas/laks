<script setup lang="ts">
const GITHUB = 'https://github.com/geyerandreas/laks';
const API = 'http://localhost:8080/api/docs/';
</script>

<template>
  <UFooter class="border-t border-default">
    <template #right>
      <nav
        class="flex flex-wrap items-center gap-x-4 gap-y-2"
        aria-label="Project links"
      >
        <UButton
          color="neutral"
          variant="link"
          size="sm"
          label="GitHub"
          icon="i-lucide-github"
          :href="GITHUB"
          target="_blank"
          rel="noopener noreferrer"
        />
        <UButton
          color="neutral"
          variant="link"
          size="sm"
          label="Swagger API"
          icon="i-lucide-book-open"
          :href="API"
        />
      </nav>
    </template>
  </UFooter>
</template>
