import { useVaultSession } from "@/context/vault-session";
import { useLocalStorage } from "@/hooks/useLocalStorage";
import { crypto } from "@/libraries/crypto";

const DEFAULT_PINNED_ROUTES_KEY = "dashboard-pin-placeholder";

export const getPinRoutesKey = (path?: string) => {
  if (path == null) return null;
  return `dashboard-pin-${crypto.sha512(path)}`;
};

export const usePinNavigation = () => {
  const { metadata } = useVaultSession();

  const [pinnedRouteUrls, setPinnedRouteUrls] = useLocalStorage<string[]>({
    key: getPinRoutesKey(metadata?.path) ?? DEFAULT_PINNED_ROUTES_KEY,
    defaultValue: [],
  });

  const togglePin = (url: string) => {
    setPinnedRouteUrls(prev =>
      prev.includes(url) ? prev.filter(p => p !== url) : [...prev, url],
    );
  };

  return { pinnedRouteUrls, togglePin };
};
