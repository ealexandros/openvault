import { nextJsConfig } from "@openvault/tooling/eslint/next";
import { defineConfig } from "eslint/config";

export default defineConfig([
  ...nextJsConfig,
  {
    rules: {
      "@next/next/no-img-element": "off",
    },
  },
  {
    ignores: [
      "src/styles/**",
      "src/components/ui/shadcn/**",
      "src-tauri/**",
      "postcss.config.mjs",
    ],
  },
]);
