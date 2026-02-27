"use client";

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
  accessAt: string;
};

export type VaultView = "selection" | "unlock";

const getVaultName = (path: string) =>
  path
    .replace(/\\/g, "/")
    .split("/")
    .pop()
    ?.replace(/\.[^/.]+$/, "") ?? "";

export const useVaultAccess = () => {
  const [view, setView] = useState<VaultView>("selection");
  const [selectedVaultPath, setSelectedVaultPath] = useState<string | null>(null);
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isLoadingVaults, setIsLoadingVaults] = useState(true);
  const [isUnlocking, setIsUnlocking] = useState(false);
  const [rememberVault, setRememberVault] = useState(true);

  const router = useRouter();
  const { setSelectedPath, setIsUnlocked } = useVault();

  const [recentVaults, setRecentVaults] = useLocalStorage<RecentVault[]>({
    key: RECENT_VAULTS_KEY,
    defaultValue: [],
  });

  const [validatedVaults, setValidatedVaults] = useState<RecentVault[]>([]);

  const validateVaults = async (vaults: RecentVault[]) => {
    const results = await Promise.all(
      vaults.map(async v => {
        const result = await tauriApi.checkPathIsFile({ path: v.path });
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

    handleConnect(selected);
  };

  const handleConnect = (path: string) => {
    setSelectedVaultPath(path);
    setView("unlock");
  };

  const handleBack = () => {
    setView("selection");
    setPassword("");
    setSelectedVaultPath(null);
  };

  const handleUnlock = async () => {
    if (!password || selectedVaultPath == null || isUnlocking) return;

    setIsUnlocking(true);

    const result = await tauriApi.openVault({
      path: selectedVaultPath,
      password,
    });

    if (!result.success) {
      toast.error("Incorrect password", {
        description: (result.error ?? "Failed to open vault") as string,
      });
      setPassword("");
      setIsUnlocking(false);
      return;
    }

    if (rememberVault) {
      const newRecent: RecentVault = {
        id: crypto.randomUUID(),
        name: getVaultName(selectedVaultPath.split("/").pop() ?? ""),
        path: selectedVaultPath,
        accessAt: new Date().toISOString(),
      };

      setRecentVaults(prev => {
        const withoutDuplicate = prev.filter(v => v.path !== selectedVaultPath);
        return [newRecent, ...withoutDuplicate].slice(0, 3);
      });
    }

    setIsUnlocked(true);
    setSelectedPath(selectedVaultPath);
    router.push(hrefs.dashboard.get());
  };

  const toggleShowPassword = () => setShowPassword(prev => !prev);

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
    view,
    recentVaults: validatedVaults,
    selectedVaultPath,
    isLoadingVaults,
    isUnlocking,
    password,
    showPassword,
    rememberVault,
    hasRecentActivity: Boolean(validatedVaults.length),
    setPassword,
    toggleShowPassword,
    handleSelect,
    handleConnect,
    handleUnlock,
    handleBack,
    handleRemoveRecent,
    handleClearRecent,
    setRememberVault,
  };
};
