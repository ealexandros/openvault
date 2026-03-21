import { describe, it, expect } from "bun:test";
import { cn } from "../../../src/utils/cn";

describe("cn", () => {
  it("should merge tailwind classes properly", () => {
    expect(cn("p-2", "p-4")).toBe("p-4");
    expect(cn("text-red-500", { "bg-blue-500": true })).toBe("text-red-500 bg-blue-500");
  });
});
