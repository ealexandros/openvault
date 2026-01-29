import { defineConfig, globalIgnores } from "eslint/config";

import nextVitals from "eslint-config-next/core-web-vitals";
import nextTypeScript from "eslint-config-next/typescript";

const config = defineConfig([
  ...nextVitals,
  ...nextTypeScript,
  {
    rules: {
      "no-console": ["warn"],
      "no-constant-condition": ["warn"],
      "no-unreachable-loop": ["error"],
      "object-shorthand": ["warn"],
      "no-implicit-coercion": ["warn"],
      "no-promise-executor-return": ["error"],
      eqeqeq: ["warn", "always", { null: "ignore" }],
    },
  },
  {
    languageOptions: {
      parserOptions: {
        project: "./tsconfig.json",
        tsconfigRootDir: __dirname,
      },
    },
    rules: {
      "@typescript-eslint/no-floating-promises": ["error"],
      "@typescript-eslint/no-misused-promises": [
        "warn",
        {
          checksVoidReturn: { attributes: false },
        },
      ],
      "@typescript-eslint/no-dynamic-delete": ["error"],
      // "@typescript-eslint/consistent-type-imports": ["error"],
      "@typescript-eslint/no-unnecessary-type-assertion": ["warn"],
      "@typescript-eslint/prefer-for-of": ["warn"],
      "@typescript-eslint/no-for-in-array": ["warn"],
      "@typescript-eslint/prefer-find": ["warn"],
      "@typescript-eslint/require-await": ["warn"],
      "@typescript-eslint/no-unnecessary-boolean-literal-compare": ["warn"],
      "@typescript-eslint/no-unnecessary-parameter-property-assignment": ["warn"],
      "@typescript-eslint/strict-boolean-expressions": ["warn"],
      "@typescript-eslint/no-unnecessary-condition": ["warn"],
      "@typescript-eslint/prefer-nullish-coalescing": ["warn"],
      "@typescript-eslint/array-type": ["warn"],
      "@typescript-eslint/consistent-type-definitions": ["warn", "type"],
    },
  },
  globalIgnores([
    ".next/**",
    "out/**",
    "build/**",
    "next-env.d.ts",
    "src/styles/**",
    "src/components/ui/shadcn/**",
  ]),
]);

export default config;
