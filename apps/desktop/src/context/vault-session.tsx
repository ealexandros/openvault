"use client";

import { queryClient } from "@/libraries/react-query";
import { tauriApi } from "@/libraries/tauri-api";
import { VaultMetaResult } from "@/types/filesystem";
import { useRouter } from "next/navigation";
import { createContext, PropsWithChildren, useContext, useState } from "react";
import { toast } from "sonner";

export type VaultMetadata = VaultMetaResult;

type VaultSessionContextType = {
  metadata: VaultMetadata | null;
  isUnlocked: boolean;
  refreshMeta: () => Promise<boolean>;
  lockVault: () => Promise<void>;
  unlockVault: () => Promise<boolean>;
};

const VaultSessionContext = createContext<VaultSessionContextType | undefined>(undefined);

export const VaultProvider = ({ children }: PropsWithChildren) => {
  const [metadata, setMetadata] = useState<VaultMetadata | null>(null);

  const router = useRouter();

  const lockVault = async () => {
    const toastId = toast.loading("Locking the vault");
    await tauriApi.lockVault();
    queryClient.clear();
    setMetadata(null);
    toast.dismiss(toastId);
    router.push("/");
  };

  const refreshMeta = async () => {
    const result = await tauriApi.getVaultMeta();
    if (result.success) setMetadata(result.data);
    return result.success;
  };

  return (
    <VaultSessionContext.Provider
      value={{
        metadata,
        isUnlocked: metadata !== null,
        refreshMeta,
        lockVault,
        unlockVault: refreshMeta,
      }}>
      {children}
    </VaultSessionContext.Provider>
  );
};

export const useVaultSession = () => {
  const context = useContext(VaultSessionContext);

  if (context === undefined) {
    throw new Error("useVault must be used within a VaultProvider");
  }

  return context;
};
