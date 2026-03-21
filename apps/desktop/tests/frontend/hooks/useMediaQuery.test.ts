import { useMediaQuery } from "@/hooks/useMediaQuery";
import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, mock } from "bun:test";

describe("useMediaQuery", () => {
  const originalMatchMedia = window.matchMedia;

  const createMediaQueryList = (
    matches: boolean,
    query: string,
    onChange?: (cb: (e: MediaQueryListEvent) => void) => void,
  ): MediaQueryList => {
    const addEventListener = mock(
      (_: string, listener: EventListenerOrEventListenerObject) => {
        if (typeof listener === "function") {
          onChange?.(listener as (e: MediaQueryListEvent) => void);
        }
      },
    ) as MediaQueryList["addEventListener"];

    return {
      matches,
      media: query,
      onchange: null,
      addListener: mock(),
      removeListener: mock(),
      addEventListener,
      removeEventListener: mock() as MediaQueryList["removeEventListener"],
      dispatchEvent: mock(() => true),
    };
  };

  beforeEach(() => {
    Object.defineProperty(window, "matchMedia", {
      writable: true,
      value: mock((query: string): MediaQueryList => createMediaQueryList(false, query)),
    });
  });

  afterEach(() => {
    window.matchMedia = originalMatchMedia;
  });

  it("should return isReady true and matchesQuery false initially", () => {
    const { result } = renderHook(() => useMediaQuery({ minWidth: 1024 }));

    expect(result.current.isReady).toBe(true);
    expect(result.current.matchesQuery).toBe(false);
  });

  it("should update matchesQuery when media query matches", () => {
    let matches = false;
    let changeHandler: (e: MediaQueryListEvent) => void = () => {};

    const mediaQueryList = createMediaQueryList(false, "(min-width: 1024px)", cb => {
      changeHandler = cb;
    });

    Object.defineProperty(window, "matchMedia", {
      value: mock(() => mediaQueryList),
    });

    const { result } = renderHook(() => useMediaQuery({ minWidth: 1024 }));

    expect(result.current.matchesQuery).toBe(false);

    act(() => {
      matches = true;
      Object.defineProperty(mediaQueryList, "matches", {
        get: () => matches,
      });

      changeHandler({ matches: true } as MediaQueryListEvent);
    });

    expect(result.current.matchesQuery).toBe(true);
  });

  it("should format media query with minWidth and maxWidth correctly", () => {
    const matchMediaMock = mock(
      (query: string): MediaQueryList => createMediaQueryList(false, query),
    );

    Object.defineProperty(window, "matchMedia", {
      value: matchMediaMock,
    });

    renderHook(() => useMediaQuery({ minWidth: 500, maxWidth: 800 }));

    expect(matchMediaMock).toHaveBeenCalled();
    const query = matchMediaMock.mock.calls[0]?.[0];
    expect(query).toContain("(min-width: 500px)");
    expect(query).toContain("and");
    expect(query).toContain("(max-width: 800px)");
  });
});
