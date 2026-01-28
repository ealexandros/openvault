import { KeyCode } from "@/config/keycodes";
import { useEffect } from "react";

type KeyHandlerMap = {
  [key in KeyCode]?: (event: KeyboardEvent) => Promise<unknown> | unknown;
};

type UseKeyListenersOptions = {
  preventDefault?: boolean;
  ignoreMeta?: boolean;
  ignoreCtrl?: boolean;
  ignoreAlt?: boolean;
  ignoreShift?: boolean;
  onlyWithoutModifiers?: boolean;
};

export const useKeyListeners = (
  listeners: KeyHandlerMap,
  options: UseKeyListenersOptions = {},
) => {
  useEffect(() => {
    const {
      preventDefault = false,
      ignoreMeta = true,
      ignoreCtrl = true,
      ignoreAlt = true,
      ignoreShift = false,
      onlyWithoutModifiers = false,
    } = options;

    const handleKeyDown = (event: KeyboardEvent) => {
      const { metaKey, ctrlKey, altKey, shiftKey, code } = event;

      const hasAnyModifier = metaKey || ctrlKey || altKey || shiftKey;

      const isModifierBlocked =
        (onlyWithoutModifiers && hasAnyModifier) ||
        (ignoreMeta && metaKey) ||
        (ignoreCtrl && ctrlKey) ||
        (ignoreAlt && altKey) ||
        (ignoreShift && shiftKey);

      if (isModifierBlocked) return;

      const listener = listeners[code as KeyCode];
      if (!listener) return;

      if (preventDefault) event.preventDefault();
      void listener(event);
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [listeners, options]);
};
