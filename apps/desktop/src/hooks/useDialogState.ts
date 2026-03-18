import { useState } from "react";

export const useDialogState = <T>() => {
  const [item, setItem] = useState<T | null>(null);

  const isOpen = item !== null;

  const open = (value: T) => setItem(value);
  const close = () => setItem(null);

  const toggle = (visible: boolean) => {
    if (!visible) setItem(null);
  };

  return {
    item,
    isOpen,
    open,
    close,
    toggle,
  };
};
