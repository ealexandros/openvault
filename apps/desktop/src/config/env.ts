import { z } from "zod";
import pkg from "../../package.json";

const formatZodErrors = (issues: z.core.$ZodIssue[]) => {
  return issues.map(({ path, message }) => `${path.join(".")} - ${message}`).join("\n");
};

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
  const messages = formatZodErrors(parsedEnv.error.issues);
  throw new Error(`❌ Invalid environment variables:\n${messages}`);
}

export const env = {
  ...parsedEnv.data,
  VERSION: pkg.version,
  IS_DEV: parsedEnv.data.NODE_ENV === "development",
  IS_PROD: parsedEnv.data.NODE_ENV === "production",
  IS_TEST: parsedEnv.data.NODE_ENV === "test",
};
