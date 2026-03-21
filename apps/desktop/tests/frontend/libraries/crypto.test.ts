import { crypto } from "@/libraries/crypto";
import { describe, expect, it } from "bun:test";

describe("crypto.sha256", () => {
  it("should return a consistent hash for the same input", () => {
    const input = "hello world";
    const hash1 = crypto.sha256(input);
    const hash2 = crypto.sha256(input);

    expect(hash1).toBe(hash2);
  });

  it("should produce a 64-character hex string", () => {
    const hash = crypto.sha256("test input");
    expect(hash).toMatch(/^[a-f0-9]{64}$/);
  });
});

describe("crypto.sha512", () => {
  it("should return a consistent hash for the same input", () => {
    const input = "hello world";
    const hash1 = crypto.sha512(input);
    const hash2 = crypto.sha512(input);

    expect(hash1).toBe(hash2);
  });

  it("should produce a 128-character hex string", () => {
    const hash = crypto.sha512("test input");
    expect(hash).toMatch(/^[a-f0-9]{128}$/);
  });

  it("should produce different hashes for different inputs", () => {
    const hash1 = crypto.sha512("input 1");
    const hash2 = crypto.sha512("input 2");
    expect(hash1).not.toBe(hash2);
  });
});
