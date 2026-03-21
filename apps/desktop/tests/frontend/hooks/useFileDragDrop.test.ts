import { useFileDragDrop } from "@/hooks/useFileDragDrop";
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, mock } from "bun:test";

declare global {
  var triggerDragEnter: ((event?: unknown) => void) | undefined;
  var triggerDragLeave: ((event?: unknown) => void) | undefined;
  var triggerDragDrop: ((event?: unknown) => Promise<void>) | undefined;
}

void mock.module("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    listen: mock((event, callback) => {
      if (event === "tauri://drag-enter") {
        Object.defineProperty(global, "triggerDragEnter", {
          value: callback,
          configurable: true,
        });
      } else if (event === "tauri://drag-leave") {
        Object.defineProperty(global, "triggerDragLeave", {
          value: callback,
          configurable: true,
        });
      } else if (event === "tauri://drag-drop") {
        Object.defineProperty(global, "triggerDragDrop", {
          value: callback,
          configurable: true,
        });
      }
      return () => {};
    }),
  }),
}));

describe("useFileDragDrop", () => {
  it("should initialize with isDragging as false", () => {
    const onDrop = mock();
    const { result } = renderHook(() => useFileDragDrop({ onDrop }));

    expect(result.current.isDragging).toBe(false);
  });

  it("should update isDragging to true when tauri://drag-enter is triggered", () => {
    const onDrop = mock();
    const { result } = renderHook(() => useFileDragDrop({ onDrop }));

    const trigger = globalThis.triggerDragEnter;

    act(() => {
      if (trigger) trigger();
    });

    expect(result.current.isDragging).toBe(true);
  });

  it("should update isDragging to false when tauri://drag-leave is triggered", () => {
    const onDrop = mock();
    const { result } = renderHook(() => useFileDragDrop({ onDrop }));

    const triggerEnter = globalThis.triggerDragEnter;
    const triggerLeave = globalThis.triggerDragLeave;

    act(() => {
      if (triggerEnter) triggerEnter();
    });
    expect(result.current.isDragging).toBe(true);

    act(() => {
      if (triggerLeave) triggerLeave();
    });
    expect(result.current.isDragging).toBe(false);
  });

  it("should call onDrop and set isDragging to false when tauri://drag-drop is triggered", async () => {
    const onDrop = mock();
    const { result } = renderHook(() => useFileDragDrop({ onDrop }));

    const triggerEnter = globalThis.triggerDragEnter;
    const triggerDrop = globalThis.triggerDragDrop;

    act(() => {
      if (triggerEnter) triggerEnter();
    });
    expect(result.current.isDragging).toBe(true);

    const mockEvent = { payload: ["file1.txt"] };

    await act(async () => {
      if (triggerDrop) await triggerDrop(mockEvent);
    });

    expect(result.current.isDragging).toBe(false);
    expect(onDrop).toHaveBeenCalledWith(mockEvent);
  });
});
