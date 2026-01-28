/* eslint-disable no-console */

import { env } from "@/config/env";

export const logger = {
  log: (...args: unknown[]) => {
    if (env.IS_DEV) console.log(...args);
  },
  warn: (...args: unknown[]) => {
    if (env.IS_DEV) console.warn(...args);
  },
  error: (...args: unknown[]) => {
    if (env.IS_DEV) console.error(...args);
  },
  info: (...args: unknown[]) => {
    if (env.IS_DEV) console.info(...args);
  },
};
