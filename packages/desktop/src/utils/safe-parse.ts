import { logger } from "@/libraries/logger";

export const safeJsonParse = <T>(value?: string | null) => {
  if (value == null) return null;

  try {
    return JSON.parse(value) as T;
  } catch (error) {
    logger.warn("⚠️ Error parsing value:", value, "Error:", error);
    return null;
  }
};

export const safeJsonStringify = <T>(value?: T) => {
  if (value == null) return null;

  try {
    return JSON.stringify(value);
  } catch (error) {
    logger.warn(`⚠️ Error stringifying value:`, value, "Error:", error);
    return null;
  }
};
