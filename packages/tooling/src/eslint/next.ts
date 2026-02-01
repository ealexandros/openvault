import type { Linter } from "eslint";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTypeScript from "eslint-config-next/typescript";
import { globalIgnores } from "eslint/config";
import { baseConfig } from "./base";

export const nextJsConfig: Linter.Config[] = [
  ...baseConfig,
  ...nextVitals,
  ...nextTypeScript,

  globalIgnores([".next/**", "out/**", "build/**", "next-env.d.ts"]),
];
