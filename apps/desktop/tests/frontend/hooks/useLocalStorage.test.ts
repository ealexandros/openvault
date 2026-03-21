import { useLocalStorage } from "@/hooks/useLocalStorage";
import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it } from "bun:test";

describe("useLocalStorage", () => {
  const KEY = "test-key";
  const DEFAULT_VALUE = "default";

  beforeEach(() => {
    window.localStorage.clear();
  });

  afterEach(() => {
    window.localStorage.clear();
  });

  it("should initialize with default value if no value exists in localStorage", () => {
    const { result } = renderHook(() =>
      useLocalStorage({ key: KEY, defaultValue: DEFAULT_VALUE }),
    );

    expect(result.current[0]).toBe(DEFAULT_VALUE);
  });

  it("should initialize with value from localStorage if it exists", () => {
    window.localStorage.setItem(KEY, JSON.stringify("stored-value"));

    const { result } = renderHook(() =>
      useLocalStorage({ key: KEY, defaultValue: DEFAULT_VALUE }),
    );

    expect(result.current[0]).toBe("stored-value");
  });

  it("should update value in localStorage when setValue is called", () => {
    const { result } = renderHook(() =>
      useLocalStorage({ key: KEY, defaultValue: DEFAULT_VALUE }),
    );

    act(() => {
      result.current[1](() => "new-value");
    });

    expect(result.current[0]).toBe("new-value");
    expect(JSON.parse(window.localStorage.getItem(KEY)!)).toBe("new-value");
  });

  it("should remove value from localStorage when removeValue is called", () => {
    window.localStorage.setItem(KEY, JSON.stringify("stored-value"));

    const { result } = renderHook(() =>
      useLocalStorage({ key: KEY, defaultValue: DEFAULT_VALUE }),
    );

    act(() => {
      result.current[2]();
    });

    expect(result.current[0]).toBe(DEFAULT_VALUE);
    expect(window.localStorage.getItem(KEY)).toBe(null);
  });

  it("should sync value when triggered by storage event", () => {
    const { result } = renderHook(() =>
      useLocalStorage({ key: KEY, defaultValue: DEFAULT_VALUE }),
    );

    act(() => {
      window.localStorage.setItem(KEY, JSON.stringify("external-change"));
      window.dispatchEvent(new Event("storage"));
    });

    expect(result.current[0]).toBe("external-change");
  });
});
