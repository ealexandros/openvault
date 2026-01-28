import { hrefs } from "@/config/hrefs";
import { useVault } from "@/context/VaultContext";
import { useLocalStorage } from "@/hooks/useLocalStorage";
import { safeAsync } from "@/utils/safe-async";
import { open } from "@tauri-apps/plugin-dialog";
import { useRouter } from "next/navigation";
import { type RecentVault } from "./_components_/RecentVaultItem";

const RECENT_VAULTS_KEY = "recent-vaults";

export const useVaultSelection = () => {
  const { selectedPath, setSelectedPath } = useVault();

  const router = useRouter();

  const [recentVaults, setRecentVaults] = useLocalStorage<RecentVault[]>({
    key: RECENT_VAULTS_KEY,
    defaultValue: [],
  });

  const select = async () => {
    const selected = await safeAsync({
      promise: open({
        title: "Select a folder to vault",
        directory: true,
        multiple: false,
      }),
      errorMessage: "Failed to open folder picker",
    });

    if (!selected || typeof selected !== "string") return;

    setSelectedPath(selected);

    const name = selected.split("/").pop() || selected;
    const newRecent: RecentVault = {
      id: crypto.randomUUID(),
      name,
      path: selected,
      lastAccessed: "Just now",
      isEncrypted: false,
    };

    setRecentVaults(prev => {
      const filtered = prev.filter(v => v.path !== selected);
      return [newRecent, ...filtered].slice(0, 3);
    });

    connect(selected);
  };

  const connect = (path: string) => {
    setSelectedPath(path);
    router.push(hrefs.unlock.get());
  };

  return {
    selectedPath,
    recentVaults,
    select,
    connect,
  };
};
