import { logger } from "@/libraries/logger";
import { safeAsync } from "@/utils/safe-async";
import { afterEach, beforeEach, describe, expect, it, vi } from "bun:test";

describe("safeAsync", () => {
  let originalAlert: unknown;
  let errorSpy: unknown;

  beforeEach(() => {
    errorSpy = vi.spyOn(logger, "error").mockImplementation(() => {});
    originalAlert = global.alert;
    Object.defineProperty(global, "alert", {
      value: vi.fn(),
      configurable: true,
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    Object.defineProperty(global, "alert", {
      value: originalAlert,
      configurable: true,
    });
  });

  it("should return the resolved value of a successful promise", async () => {
    const promise = Promise.resolve("success data");
    const result = await safeAsync({ promise });
    expect(result).toBe("success data");
  });

  it("should catch errors, log them, and return null", async () => {
    const error = new Error("Test error");
    const promise = Promise.reject(error);

    const result = await safeAsync({ promise });

    expect(result).toBeNull();
    expect(errorSpy).toHaveBeenCalledWith("⚠️ Async operation failed", error);
  });

  it("should use the provided custom errorMessage", async () => {
    const error = new Error("Test error");
    const promise = Promise.reject(error);
    const customMessage = "Custom error message";

    const result = await safeAsync({ promise, errorMessage: customMessage });

    expect(result).toBeNull();
    expect(errorSpy).toHaveBeenCalledWith(customMessage, error);
  });

  it("should trigger an alert if alertMessage is provided and an error occurs", async () => {
    const error = new Error("Test error");
    const promise = Promise.reject(error);
    const alertMessage = "Show this to the user";

    await safeAsync({ promise, alertMessage });

    expect(global.alert).toHaveBeenCalledWith(alertMessage);
  });
});
