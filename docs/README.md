# Laks documentation

This directory contains the Laks product and developer documentation, built with
Docus and Nuxt Content.

## Run locally

```sh
cd docs
bun install
bun run dev
```

Open the local URL printed by Nuxt (usually `http://localhost:3000`).

## Write content

Pages live in `content/`. The numbered directory and file names control sidebar
ordering; each page's front matter supplies its title and description. The
homepage is `content/index.md`.

## Build

```sh
cd docs
bun run build
```

The production output is written to `.output/`.

## Code quality

```sh
cd docs
bun run lint
bun run format
```

Use `bun run format:check` to check formatting without changing files.
