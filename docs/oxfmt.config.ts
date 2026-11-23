import { defineConfig } from "oxfmt";

export default defineConfig({
  printWidth: 80,
  sortPackageJson: true,
  trailingComma: "all",
  semi: true,
  ignorePatterns: [
    ".data",
    ".nuxt",
    ".output",
    ".nitro",
    ".cache",
    "dist",
    "node_modules",
    "bun.lock",
    "content/**/*",
  ],
});
