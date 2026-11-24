# UI development

The frontend is a Vue 3 single-page application built with Vite, TypeScript, and Bun.

```sh
cd ui
bun install
bun run dev
```

The development server uses port 3000 and connects to the API at `http://localhost:8080` by default. Set `VITE_API_BASE` before starting or building to use a different API origin. The API must allow credentialed requests from the frontend origin.

Production builds are written to `dist/`. Configure the static server to serve `index.html` for client-side routes, including `/login`, `/packages`, and `/settings/users`.

```sh
bun run lint
bun run format:check
bun run typecheck
bun run test:unit
bun run build
```
