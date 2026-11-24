<template>
  <RouterLink
    to="/"
    aria-label="Laks home"
    class="brand rounded-md text-highlighted focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-primary"
  >
    <span
      class="logo-mark flex size-9 shrink-0 items-center justify-center rounded-xl bg-primary text-inverted"
    >
      <span class="fish-mark" aria-hidden="true">&gt;&lt;(°&gt;</span>
    </span>

    <span class="wordmark">laks</span>
  </RouterLink>
</template>

<style scoped>
.brand {
  display: inline-flex;
  align-items: center;
  gap: 0.625rem;
  text-decoration: none;
}

.logo-mark {
  box-shadow: 0 2px 6px color-mix(in srgb, var(--ui-primary) 24%, transparent);
}

.fish-mark {
  font-family: 'SFMono-Regular', Consolas, monospace;
  font-size: 0.6875rem;
  font-weight: 700;
  letter-spacing: -0.1em;
  white-space: nowrap;
}

.wordmark {
  font-size: 1.25rem;
  font-weight: 800;
  letter-spacing: -0.045em;
  line-height: 1;
}
</style>
