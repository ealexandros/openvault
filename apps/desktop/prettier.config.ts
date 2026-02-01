import { baseConfig } from "@openvault/tooling/prettier";
import { type Config } from "prettier";

const config: Config = {
  ...baseConfig,
  tailwindStylesheet: "./src/styles/globals.css",
};

export default config;
