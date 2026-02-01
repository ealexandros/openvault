"use client";

import { useRouter } from "next/navigation";
import { createContext, PropsWithChildren, useContext, useState } from "react";

type VaultContextType = {
  selectedPath: string | null;
  setSelectedPath: (path: string | null) => void;
  isUnlocked: boolean;
  setIsUnlocked: (unlocked: boolean) => void;
  vaultName: string | undefined;
  lockVault: () => void;
};

const VaultContext = createContext<VaultContextType | undefined>(undefined);

export const VaultProvider = ({ children }: PropsWithChildren) => {
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [isUnlocked, setIsUnlocked] = useState(false);

  const vaultName = selectedPath?.split(/[/\\]/).pop();

  const router = useRouter();

  const lockVault = () => {
    setIsUnlocked(false);
    setSelectedPath(null);
    router.push("/");
  };

  return (
    <VaultContext.Provider
      value={{
        selectedPath,
        setSelectedPath,
        isUnlocked,
        setIsUnlocked,
        vaultName,
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
