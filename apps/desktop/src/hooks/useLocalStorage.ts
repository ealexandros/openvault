import { safeJsonParse } from "@/utils/safe-parse";
import { useEffect, useSyncExternalStore } from "react";

type UseLocalStorageOptions<T> = {
  key: string;
  defaultValue: T;
};

const localStorageSubscribers = new Set<() => void>();

const subscribe = (callback: () => void) => {
  localStorageSubscribers.add(callback);
  return () => localStorageSubscribers.delete(callback);
};

const notify = () => {
  localStorageSubscribers.forEach(callback => callback());
};

export const useLocalStorage = <T = unknown>({
  key,
  defaultValue,
}: UseLocalStorageOptions<T>) => {
  const getSnapshot = () => {
    if (typeof window === "undefined") return null;
    return window.localStorage.getItem(key);
  };

  const getServerSnapshot = () => null;

  const storageValue = useSyncExternalStore(subscribe, getSnapshot, getServerSnapshot);

  const value = safeJsonParse<T>(storageValue) ?? defaultValue;

  const setValue = (callback: (value: T) => T) => {
    const newValue = callback(value);
    if (typeof window === "undefined") return;
    window.localStorage.setItem(key, JSON.stringify(newValue));
    notify();
  };

  const removeValue = () => {
    if (typeof window === "undefined") return;
    window.localStorage.removeItem(key);
    notify();
  };

  useEffect(() => {
    if (typeof window === "undefined") return;

    const handleStorage = () => notify();

    window.addEventListener("storage", handleStorage);
    window.addEventListener("focus", notify);
    document.addEventListener("visibilitychange", notify);

    return () => {
      window.removeEventListener("storage", handleStorage);
      window.removeEventListener("focus", notify);
      document.removeEventListener("visibilitychange", notify);
    };
  }, []);

  return [value, setValue, removeValue] as const;
};
