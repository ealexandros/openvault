"use client";

import { useState } from "react";

export type VaultView = "selection" | "unlock";

export const useVaultAccess = () => {
  const [view, setView] = useState<VaultView>("selection");
  const [selectedVaultPath, setSelectedVaultPath] = useState<string | null>(null);

  const handleConnect = (path: string) => {
    setSelectedVaultPath(path);
    setView("unlock");
  };

  const handleBack = () => {
    setView("selection");
    setSelectedVaultPath(null);
  };

  return {
    view,
    selectedVaultPath,
    handleConnect,
    handleBack,
  };
};
