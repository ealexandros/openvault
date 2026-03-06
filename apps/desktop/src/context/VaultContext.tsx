"use client";

import { tauriApi } from "@/libraries/tauri-api";
import { useRouter } from "next/navigation";
import { createContext, PropsWithChildren, useContext, useState } from "react";
import { toast } from "sonner";

type VaultContextType = {
  isUnlocked: boolean;
  selectedPath: string | null;
  vaultName: string | undefined;
  setSelectedPath: (path: string | null) => void;
  setIsUnlocked: (unlocked: boolean) => void;
  lockVault: () => Promise<void>;
};

const VaultContext = createContext<VaultContextType | undefined>(undefined);

export const VaultProvider = ({ children }: PropsWithChildren) => {
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [isUnlocked, setIsUnlocked] = useState(false);

  const vaultName = selectedPath?.split(/[/\\]/).pop();

  const router = useRouter();

  const lockVault = async () => {
    const toastId = toast.loading("Locking the vault");
    await tauriApi.lockVault();
    setIsUnlocked(false);
    setSelectedPath(null);
    toast.dismiss(toastId);

    router.push("/");
  };

  return (
    <VaultContext.Provider
      value={{
        isUnlocked,
        selectedPath,
        vaultName,
        setSelectedPath,
        setIsUnlocked,
        lockVault,
      }}>
      {children}
    </VaultContext.Provider>
  );
};

export const useVault = () => {
  const context = useContext(VaultContext);

  if (context === undefined) {
    throw new Error("useVault must be used within a VaultProvider");
  }

  return context;
};
