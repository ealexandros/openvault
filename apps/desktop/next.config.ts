import path from "path";
import { env } from "./src/config/env";

const assetPrefix = env.IS_PROD ? undefined : `http://${env.TAURI_DEV_HOST}:8080`;
const rootPath = path.resolve(__dirname, "../..");

const nextConfig = {
  output: "export",
  reactCompiler: true,
  devIndicators: false,
  assetPrefix,
  images: {
    unoptimized: true,
  },
  turbopack: {
    root: rootPath,
  },
};

export default nextConfig;
