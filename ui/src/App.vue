<script setup lang="ts">
import { useColorMode } from '@vueuse/core';
import { RouterView } from 'vue-router';
import AppHeader from './components/AppHeader.vue';

useColorMode({ initialValue: 'dark' });
</script>

<template>
  <UApp>
    <AppHeader />
    <UMain>
      <UContainer class="py-8">
        <RouterView />
      </UContainer>
    </UMain>
    <UFooter />
  </UApp>
</template>
