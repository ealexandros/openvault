import { useResponsive } from "@/hooks/useResponsive";
import { renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, Mock, mock } from "bun:test";

describe("useResponsive", () => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let matchMediaMock: Mock<(query: string) => any>;

  beforeEach(() => {
    matchMediaMock = mock((query: string) => ({
      matches: query.includes("768"),
      media: query,
      onchange: null,
      addEventListener: mock(),
      removeEventListener: mock(),
      dispatchEvent: mock(() => true),
    }));

    window.matchMedia = matchMediaMock;
  });

  afterEach(() => {
    matchMediaMock.mockRestore();
  });

  it("should return correct responsive state based on viewport width", () => {
    const { result } = renderHook(() => useResponsive());

    expect(result.current.isReady).toBe(true);
    expect(result.current.isMd).toBe(true);
  });

  it("should mark isMobile true below mobile breakpoint", () => {
    matchMediaMock.mockImplementation((query: string) => ({
      matches: query.includes("767"),
      media: query,
      addEventListener: mock(),
      removeEventListener: mock(),
    }));

    const { result } = renderHook(() => useResponsive());
    expect(result.current.isMobile).toBe(true);
  });
});
