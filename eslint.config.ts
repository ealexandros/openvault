import { baseConfig } from "@openvault/tooling/eslint/base";
import { defineConfig } from "eslint/config";

export default defineConfig([
  ...baseConfig,
  {
    ignores: [".lintstagedrc.js", ".prettierrc.js", "eslint.config.ts"],
  },
]);
