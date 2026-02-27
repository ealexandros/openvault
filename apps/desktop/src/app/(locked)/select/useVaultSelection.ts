import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { useLocalStorage } from "@/hooks/useLocalStorage";
import { tauriApi } from "@/libraries/tauri-api";
import { safeAsync } from "@/utils/safe-async";
import { open } from "@tauri-apps/plugin-dialog";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { toast } from "sonner";

const RECENT_VAULTS_KEY = "recent-vaults";

export type RecentVault = {
  id: string;
  name: string;
  path: string;
  lastAccessed: string;
};

export const useVaultSelection = () => {
  const { selectedPath: globalPath, setSelectedPath } = useVault();
  const [isNavigating, setIsNavigating] = useState(false);

  const router = useRouter();

  const [recentVaults, setRecentVaults] = useLocalStorage<RecentVault[]>({
    key: RECENT_VAULTS_KEY,
    defaultValue: [],
  });

  const select = async () => {
    const toastId = toast.loading("Selecting a file...");

    const selected = await safeAsync({
      promise: open({
        title: "Select a file to vault",
        directory: false,
        multiple: false,
      }),
      errorMessage: "Failed to open file picker",
    });

    toast.dismiss(toastId);

    if (selected == null || typeof selected !== "string") return;

    setIsNavigating(true);

    const name = selected.split("/").pop() ?? selected;
    const newRecent: RecentVault = {
      id: crypto.randomUUID(),
      name,
      path: selected,
      lastAccessed: new Date().toISOString(),
    };

    setRecentVaults(prev => {
      const filtered = prev.filter(v => v.path !== selected);
      return [newRecent, ...filtered].slice(0, 3);
    });

    connect(selected);
  };

  const connect = (path: string) => {
    setIsNavigating(true);
    router.push(hrefs.unlock.get());
    setSelectedPath(path);
  };

  const [validatedVaults, setValidatedVaults] = useState<RecentVault[]>([]);

  useEffect(() => {
    const validateVaults = async () => {
      const results = await Promise.all(
        recentVaults.map(async v => {
          const result = await tauriApi.checkPathIsFile({
            path: v.path,
          });
          return result.success && result.data ? v : null;
        }),
      );

      setValidatedVaults(results.filter(Boolean) as RecentVault[]);
    };

    void validateVaults();
  }, [recentVaults]);

  const [prevPath, setPrevPath] = useState(globalPath);

  if (globalPath !== prevPath && !isNavigating) {
    setPrevPath(globalPath);
  }

  return {
    selectedPath: isNavigating ? prevPath : globalPath,
    recentVaults: validatedVaults,
    select,
    connect,
    isNavigating,
  };
};
