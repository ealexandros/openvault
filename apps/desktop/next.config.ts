import path from "path";
import { env } from "./src/config/env";

const nextConfig = {
  output: "export",

  images: {
    unoptimized: true,
  },
  assetPrefix: env.IS_PROD ? undefined : `http://${env.TAURI_DEV_HOST}:8080`,

  turbopack: {
    root: path.resolve(__dirname, "../.."),
  },

  devIndicators: false,
};

export default nextConfig;
