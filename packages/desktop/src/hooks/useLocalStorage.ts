import { safeJsonParse, safeJsonStringify } from "@/utils/safe-parse";
import { useEffect, useState } from "react";

type UseLocalStorageProps<T> = {
  key: string;
  defaultValue: T;
};

type UseLocalStorageReturn<T> = [T, React.Dispatch<React.SetStateAction<T>>];

export const useLocalStorage = <T>({
  key,
  defaultValue,
}: UseLocalStorageProps<T>): UseLocalStorageReturn<T> => {
  const [storedValue, setStoredValue] = useState<T>(defaultValue);
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    const item = window.localStorage.getItem(key);
    const value = item != null ? safeJsonParse<T>(item) : null;

    if (value !== null) {
      setStoredValue(value);
    }

    setIsInitialized(true);
  }, [key]);

  useEffect(() => {
    if (!isInitialized) return;

    if (storedValue == null) {
      window.localStorage.removeItem(key);
      return;
    }

    const serialized = safeJsonStringify(storedValue);

    if (serialized != null) {
      window.localStorage.setItem(key, serialized);
    }
  }, [isInitialized, key, storedValue]);

  return [storedValue, setStoredValue];
};
