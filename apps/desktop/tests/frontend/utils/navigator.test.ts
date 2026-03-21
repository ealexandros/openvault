import { isMacintosh } from "@/utils/navigator";
import { afterEach, beforeEach, describe, expect, it } from "bun:test";

describe("isMacintosh", () => {
  let originalNavigator: Navigator;

  beforeEach(() => {
    originalNavigator = global.navigator;
  });

  afterEach(() => {
    Object.defineProperty(global, "navigator", {
      value: originalNavigator,
      configurable: true,
    });
  });

  it("should return true when userAgent includes Mac", () => {
    Object.defineProperty(global, "navigator", {
      value: { userAgent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)" },
      configurable: true,
    });
    expect(isMacintosh()).toBe(true);
  });

  it("should return false when userAgent does not include Mac", () => {
    Object.defineProperty(global, "navigator", {
      value: { userAgent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)" },
      configurable: true,
    });
    expect(isMacintosh()).toBe(false);
  });
});
