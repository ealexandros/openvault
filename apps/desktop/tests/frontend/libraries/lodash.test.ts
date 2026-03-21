import { lodash } from "@/libraries/lodash";
import { describe, expect, it } from "bun:test";

describe("lodash.percent", () => {
  it("should return 0 if total is 0", () => {
    expect(lodash.percent(50, 0)).toBe(0);
    expect(lodash.percent(50, -10)).toBe(0);
  });

  it("should return 0 if value is 0", () => {
    expect(lodash.percent(0, 100)).toBe(0);
  });

  it("should return 100 if value equals total", () => {
    expect(lodash.percent(100, 100)).toBe(100);
  });

  it("should return correct percentage for value < total", () => {
    expect(lodash.percent(50, 200)).toBe(25);
    expect(lodash.percent(30, 120)).toBe(25);
  });

  it("should clamp percentage above 100 to 100", () => {
    expect(lodash.percent(150, 100)).toBe(100);
  });

  it("should clamp percentage below 0 to 0", () => {
    expect(lodash.percent(-50, 100)).toBe(0);
  });
});
