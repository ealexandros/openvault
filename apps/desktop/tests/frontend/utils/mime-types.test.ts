import { getFileType, getFileTypeOrDefault, getMimeType } from "@/utils/mime-types";
import { describe, expect, it } from "bun:test";

describe("getFileType", () => {
  it("should return null for undefined", () => {
    expect(getFileType(undefined)).toBeNull();
  });

  it("should ignore case when checking extensions", () => {
    expect(getFileType("PDF")).toBe("pdf");
    expect(getFileType("PnG")).toBe("image");
  });

  it("should correctly identify images", () => {
    expect(getFileType("png")).toBe("image");
    expect(getFileType("jpg")).toBe("image");
    expect(getFileType("svg")).toBe("image");
  });

  it("should correctly identify audio", () => {
    expect(getFileType("mp3")).toBe("audio");
    expect(getFileType("wav")).toBe("audio");
  });

  it("should correctly identify video", () => {
    expect(getFileType("mp4")).toBe("video");
    expect(getFileType("mkv")).toBe("video");
  });

  it("should return text for unknown extensions", () => {
    expect(getFileType("txt")).toBe("text");
    expect(getFileType("unknown")).toBe("text");
  });
});

describe("getFileTypeOrDefault", () => {
  it("should return text if extension is undefined", () => {
    expect(getFileTypeOrDefault(undefined)).toBe("text");
  });

  it("should return the correct type for known extensions", () => {
    expect(getFileTypeOrDefault("png")).toBe("image");
    expect(getFileTypeOrDefault("mp4")).toBe("video");
  });
});

describe("getMimeType", () => {
  it("should return an empty string if kind or extension is falsy", () => {
    expect(getMimeType("image", undefined)).toBe("");
    expect(getMimeType(null, "png")).toBe("");
  });

  it("should return specific mime types from the constant map", () => {
    expect(getMimeType("audio", "mp3")).toBe("audio/mpeg");
    expect(getMimeType("video", "mp4")).toBe("video/mp4");
    expect(getMimeType("image", "svg")).toBe("image/svg+xml");
    expect(getMimeType("pdf", "pdf")).toBe("application/pdf");
  });

  it("should fallback to generic MIME format if not explicitly listed", () => {
    expect(getMimeType("image", "png")).toBe("image/png");
    expect(getMimeType("audio", "custom")).toBe("audio/custom");
  });
});
