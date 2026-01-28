import { z } from "zod";
import pkg from "../../package.json";

const envSchema = z.object({
  NODE_ENV: z.enum(["development", "test", "production"]).default("development"),
  TAURI_DEV_HOST: z.string().default("localhost"),
});

const processEnv = {
  NODE_ENV: process.env.NODE_ENV,
  TAURI_DEV_HOST: process.env.TAURI_DEV_HOST,
};

const parsedEnv = envSchema.safeParse(processEnv);

if (!parsedEnv.success) {
  throw new Error(`❌ Invalid environment variables: ${z.treeifyError(parsedEnv.error)}`);
}

export const env = {
  ...parsedEnv.data,
  VERSION: pkg.version,
  IS_DEV: parsedEnv.data.NODE_ENV === "development",
  IS_PROD: parsedEnv.data.NODE_ENV === "production",
  IS_TEST: parsedEnv.data.NODE_ENV === "test",
};
