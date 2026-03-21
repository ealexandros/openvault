import { useDialogState } from "@/hooks/useDialogState";
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "bun:test";

describe("useDialogState", () => {
  it("should initialize with null item and isOpen false", () => {
    const { result } = renderHook(() => useDialogState<string>());

    expect(result.current.item).toBe(null);
    expect(result.current.isOpen).toBe(false);
  });

  it("should open with an item and set isOpen to true", () => {
    const { result } = renderHook(() => useDialogState<string>());

    act(() => {
      result.current.open("test");
    });

    expect(result.current.item).toBe("test");
    expect(result.current.isOpen).toBe(true);
  });

  it("should close and set item to null and isOpen to false", () => {
    const { result } = renderHook(() => useDialogState<string>());

    act(() => {
      result.current.open("test");
      result.current.close();
    });

    expect(result.current.item).toBe(null);
    expect(result.current.isOpen).toBe(false);
  });

  it("should toggle visibility correctly", () => {
    const { result } = renderHook(() => useDialogState<string>());

    act(() => {
      result.current.open("test");
      result.current.toggle(false);
    });

    expect(result.current.item).toBe(null);
    expect(result.current.isOpen).toBe(false);

    act(() => {
      result.current.open("test");
      result.current.toggle(true);
    });

    expect(result.current.item).toBe("test");
    expect(result.current.isOpen).toBe(true);
  });
});
