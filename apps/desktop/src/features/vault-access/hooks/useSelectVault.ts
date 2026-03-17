import { safeAsync } from "@/utils/safe-async";
import { open } from "@tauri-apps/plugin-dialog";
import { useRecentVault } from "./useRecentVault";

export const useSelectVault = (onConnect: (path: string) => void) => {
  const { recentVaults, setRecentVaults, isLoadingVaults } = useRecentVault();

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

  const handleRemoveRecent = (id: string) => {
    setRecentVaults(prev => prev.filter(v => v.id !== id));
  };

  const handleClearRecent = () => {
    setRecentVaults([]);
  };

  return {
    recentVaults,
    isLoadingVaults,
    handleSelect,
    handleRemoveRecent,
    handleClearRecent,
  };
};
