import { FlatCompat } from "@eslint/eslintrc";
import { Linter } from "eslint";
import { dirname } from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const config: Linter.Config[] = [
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
  ...compat.config({
    parser: "@typescript-eslint/parser",
    parserOptions: {
      project: "./tsconfig.json",
      tsconfigRootDir: __dirname,
    },
    plugins: ["@typescript-eslint"],
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
  }),
  ...compat.extends("next/core-web-vitals", "next/typescript"),
];

export default config;
