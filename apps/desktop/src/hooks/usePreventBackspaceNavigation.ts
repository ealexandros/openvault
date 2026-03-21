import { useHotkey } from "@tanstack/react-hotkeys";

export const usePreventBackspaceNavigation = () => {
  useHotkey("Backspace", e => {
    const activeElement = document.activeElement as HTMLElement | null;
    const isInput = activeElement?.tagName === "INPUT";
    const isTextarea = activeElement?.tagName === "TEXTAREA";
    const isContentEditable = activeElement?.isContentEditable;

    if (!isInput && !isTextarea && isContentEditable !== true) {
      e.preventDefault();
    }
  });
};
