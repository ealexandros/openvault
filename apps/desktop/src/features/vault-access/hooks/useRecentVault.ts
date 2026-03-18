import { useLocalStorage } from "@/hooks/useLocalStorage";
import { tauriApi } from "@/libraries/tauri-api";
import { useEffect, useState } from "react";
import { getVaultName } from "./useUnlockVault";

export const RECENT_VAULTS_KEY = "recent-vaults";

export type RecentVaultProps = {
  id: string;
  name: string;
  path: string;
};

const MAX_RECENT_VAULTS = 2;

export const useRecentVault = () => {
  const [validatedVaults, setValidatedVaults] = useState<RecentVaultProps[]>([]);
  const [isLoadingVaults, setIsLoadingVaults] = useState(true);

  const [recentVaults, setRecentVaults] = useLocalStorage<RecentVaultProps[]>({
    key: RECENT_VAULTS_KEY,
    defaultValue: [],
  });

  const validateVaults = async (vaults: RecentVaultProps[]) => {
    const results = await Promise.all(
      vaults.map(async v => {
        const result = await tauriApi.isFile({ path: v.path });
        return result.success && result.data ? v : null;
      }),
    );

    return results.filter((v): v is RecentVaultProps => v !== null);
  };

  const addVaultToRecents = (path: string) => {
    if (recentVaults.some(v => v.path === path)) {
      return;
    }

    setRecentVaults(prev => {
      const newRecent: RecentVaultProps = {
        id: crypto.randomUUID(),
        name: getVaultName(path.split("/").pop() ?? ""),
        path,
      };
      return [newRecent, ...prev].slice(0, MAX_RECENT_VAULTS);
    });
  };

  useEffect(() => {
    const run = async () => {
      setIsLoadingVaults(true);
      const valid = await validateVaults(recentVaults);
      setValidatedVaults(valid);
      setIsLoadingVaults(false);
    };
    void run();
  }, [recentVaults]);

  return {
    recentVaults: validatedVaults,
    isLoadingVaults,
    addVaultToRecents,
    setRecentVaults,
  };
};
