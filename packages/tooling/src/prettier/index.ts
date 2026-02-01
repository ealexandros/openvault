import type { Config } from "prettier";

export const baseConfig: Config = {
  plugins: ["prettier-plugin-tailwindcss"],
  semi: true,
  tabWidth: 2,
  singleQuote: false,
  jsxSingleQuote: false,
  arrowParens: "avoid",
  bracketSameLine: true,
  bracketSpacing: true,
  trailingComma: "all",
  printWidth: 95,
  tailwindAttributes: ["/*ClassName/"],
};
