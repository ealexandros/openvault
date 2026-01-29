import { logger } from "@/libraries/logger";

type SafeAsyncProps<T> = {
  promise: Promise<T>;
  alertMessage?: string;
  errorMessage?: string;
};

export const safeAsync = async <T>({
  promise,
  alertMessage,
  errorMessage = "⚠️ Async operation failed",
}: SafeAsyncProps<T>) => {
  try {
    return await promise;
  } catch (error) {
    logger.error(errorMessage, error);
    if (alertMessage != null) alert(alertMessage);
    return null;
  }
};
