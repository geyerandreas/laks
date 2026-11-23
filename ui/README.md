# UI development

The frontend uses Nuxt, Vue, TypeScript, and Bun.

## Setup

From the repository root, install the frontend dependencies and start Nuxt:

```sh
cd ui
bun install
bun run dev
```

## Checks and formatting

Run these commands from `ui/`:

```sh
bun run lint          # Check with ESLint
bun run lint:fix      # Apply ESLint fixes
bun run format:check  # Check formatting with Oxfmt
bun run format        # Apply Oxfmt formatting
bun run typecheck     # Check TypeScript and Vue types
```
