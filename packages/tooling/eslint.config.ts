import { defineConfig } from "eslint/config";
import { baseConfig } from "./src/eslint/base";

export default defineConfig([
  ...baseConfig,
  {
    files: ["**/*.ts", "**/*.tsx"],
    ignores: ["**/node_modules/**"],
  },
]);
