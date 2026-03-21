import { beforeEach, vi } from "bun:test";
import { Window } from "happy-dom";

beforeEach(() => {
  const window = new Window();

  Object.defineProperty(globalThis, "window", {
    value: window,
    configurable: true,
  });
  Object.defineProperty(globalThis, "document", {
    value: window.document,
    configurable: true,
  });
  Object.defineProperty(globalThis, "navigator", {
    value: window.navigator,
    configurable: true,
  });

  vi.clearAllMocks();
});
