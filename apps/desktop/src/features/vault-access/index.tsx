"use client";

import { CenterLayout } from "@/components/layout/center";
import { SelectionView } from "./components/selection/SelectionView";
import { UnlockView } from "./components/unlock/UnlockView";
import { useVaultAccess } from "./hooks/useVaultAccess";

export { useRecentVault, type RecentVaultProps } from "./hooks/useRecentVault";

export const VaultAccessScreen = () => {
  const { view, selectedVaultPath, handleConnect, handleBack } = useVaultAccess();

  return (
    <CenterLayout className="overflow-hidden p-6 sm:p-12">
      <main className="w-full max-w-xl">
        {view === "selection" ? (
          <SelectionView onConnect={handleConnect} />
        ) : (
          <UnlockView selectedVaultPath={selectedVaultPath} onBack={handleBack} />
        )}
      </main>
    </CenterLayout>
  );
};
