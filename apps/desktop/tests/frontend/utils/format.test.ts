import { formatBytes, formatFromIsoString, truncateLeft } from "@/utils/format";
import { describe, expect, it } from "bun:test";

describe("formatFromIsoString", () => {
  it("should format valid ISO strings correctly", () => {
    const raw = "2023-10-10 14:30:00.123 UTC";
    const formatted = formatFromIsoString(raw);
    expect(formatted).not.toBe(raw);
    expect(formatted).toContain("2023");
  });

  it("should return the original string if parsing fails", () => {
    const invalid = "invalid date string";
    expect(formatFromIsoString(invalid)).toBe(invalid);
  });
});

describe("formatBytes", () => {
  it("should handle 0 bytes", () => {
    expect(formatBytes(0)).toBe("0 Bytes");
  });

  it("should format various byte sizes", () => {
    expect(formatBytes(1024)).toBe("1 KB");
    expect(formatBytes(1048576)).toBe("1 MB");
    expect(formatBytes(1073741824)).toBe("1 GB");
  });

  it("should respect decimal places", () => {
    expect(formatBytes(1500, 2)).toBe("1.46 KB");
    expect(formatBytes(1500, 0)).toBe("1 KB");
  });
});

describe("truncateLeft", () => {
  it("should not truncate if length is within maxLength", () => {
    expect(truncateLeft("hello", 10)).toBe("hello");
    expect(truncateLeft("hello", 5)).toBe("hello");
  });

  it("should truncate and prefix with .. if length exceeds maxLength", () => {
    expect(truncateLeft("helloworld", 5)).toBe("..world");
  });
});
