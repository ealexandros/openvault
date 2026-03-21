import { logger } from "@/libraries/logger";
import { safeJsonParse, safeJsonStringify, safeUint8ArrayParse } from "@/utils/safe-parse";
import { afterEach, beforeEach, describe, expect, it, vi } from "bun:test";

void vi.mock("@/shared/config/env", () => ({
  sharedEnv: {},
}));

describe("safeJsonParse", () => {
  beforeEach(() => {
    vi.spyOn(logger, "warn").mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should return parsed object for valid JSON", () => {
    const json = '{"a":1,"b":2}';
    const result = safeJsonParse(json);
    expect(result).toEqual({ a: 1, b: 2 });
  });

  it("should return null for undefined", () => {
    expect(safeJsonParse(undefined)).toBeNull();
  });

  it("should return null for null input", () => {
    expect(safeJsonParse(null)).toBeNull();
  });

  it("should return null and log warning for invalid JSON", () => {
    const invalid = '{"a":1,';
    const warnSpy = vi.spyOn(logger, "warn");
    const result = safeJsonParse(invalid);
    expect(result).toBeNull();
    expect(warnSpy).toHaveBeenCalledWith(
      expect.stringContaining("⚠️ Error parsing value:"),
      invalid,
      expect.stringContaining("Error:"),
      expect.anything(),
    );
  });
});

describe("safeJsonStringify", () => {
  beforeEach(() => {
    vi.spyOn(logger, "warn").mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should return stringified JSON for valid object", () => {
    const obj = { a: 1, b: 2 };
    const result = safeJsonStringify(obj);
    expect(result).toBe(JSON.stringify(obj));
  });

  it("should return null for undefined", () => {
    expect(safeJsonStringify(undefined)).toBeNull();
  });

  it("should return null for null", () => {
    expect(safeJsonStringify(null)).toBeNull();
  });

  it("should return null and log warning for circular reference", () => {
    const obj: Record<string, unknown> = {};
    obj.self = obj;

    const warnSpy = vi.spyOn(logger, "warn");
    const result = safeJsonStringify(obj);

    expect(result).toBeNull();
    expect(warnSpy).toHaveBeenCalledWith(
      expect.stringContaining("⚠️ Error stringifying value:"),
      obj,
      expect.stringContaining("Error:"),
      expect.anything(),
    );
  });
});

describe("safeUint8ArrayParse", () => {
  beforeEach(() => {
    vi.spyOn(logger, "warn").mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should return null for null/undefined", () => {
    expect(safeUint8ArrayParse(null)).toBeNull();
    expect(safeUint8ArrayParse(undefined)).toBeNull();
  });

  it("should parse valid Uint8Array to string", () => {
    const array = new Uint8Array([104, 101, 108, 108, 111]);
    expect(safeUint8ArrayParse(array)).toBe("hello");
  });

  it("should return null and log warning for invalid Uint8Array", () => {
    const invalid = new Uint8Array([255, 255, 255]);

    const warnSpy = vi.spyOn(logger, "warn");
    const result = safeUint8ArrayParse(invalid);

    expect(result).toBeNull();
    expect(warnSpy).toHaveBeenCalledWith(
      expect.anything(),
      invalid,
      expect.anything(),
      expect.anything(),
    );
  });
});
