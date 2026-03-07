import { useLocalStorage } from "@/hooks/useLocalStorage";
import { tauriApi } from "@/libraries/tauri-api";
import { safeAsync } from "@/utils/safe-async";
import { open } from "@tauri-apps/plugin-dialog";
import { useEffect, useState } from "react";

export const RECENT_VAULTS_KEY = "recent-vaults";

export type RecentVault = {
  id: string;
  name: string;
  path: string;
  accessAt: string;
};

export const useSelectVault = (onConnect: (path: string) => void) => {
  const [recentVaults, setRecentVaults] = useLocalStorage<RecentVault[]>({
    key: RECENT_VAULTS_KEY,
    defaultValue: [],
  });

  const [validatedVaults, setValidatedVaults] = useState<RecentVault[]>([]);
  const [isLoadingVaults, setIsLoadingVaults] = useState(true);

  const validateVaults = async (vaults: RecentVault[]) => {
    const results = await Promise.all(
      vaults.map(async v => {
        const result = await tauriApi.isFile({ path: v.path });
        return result.success && result.data ? v : null;
      }),
    );

    return results.filter((v): v is RecentVault => v !== null);
  };

  const handleSelect = async () => {
    const selected = await safeAsync({
      promise: open({
        title: "Select a file to vault",
        directory: false,
        multiple: false,
        filters: [{ name: "Vault", extensions: ["ov"] }],
      }),
      errorMessage: "Failed to open file picker",
    });

    if (typeof selected !== "string") return;

    onConnect(selected);
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

  const handleRemoveRecent = (id: string) => {
    setRecentVaults(prev => prev.filter(v => v.id !== id));
  };

  const handleClearRecent = () => {
    setRecentVaults([]);
  };

  return {
    recentVaults: validatedVaults,
    isLoadingVaults,
    handleSelect,
    handleRemoveRecent,
    handleClearRecent,
  };
};
