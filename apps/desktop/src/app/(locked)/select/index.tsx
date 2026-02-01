"use client";

import { CenterLayout } from "@/components/layout/center";
import { ActionSection } from "./_components_/ActionSection";
import { RecentVaultsList } from "./_components_/RecentVaultsList";
import { VaultHeader } from "./_components_/VaultHeader";
import { useVaultSelection } from "./useVaultSelection";

export const VaultSelectionPage = () => {
  const { selectedPath, recentVaults, select, connect } = useVaultSelection();

  return (
    <CenterLayout className="p-6 selection:bg-primary/30 sm:p-12">
      <main className="w-full max-w-lg space-y-10">
        <VaultHeader />
        <ActionSection
          selectedPath={selectedPath}
          onSelectFolder={select}
          onConnect={connect}
        />
        <RecentVaultsList vaults={recentVaults} onConnect={connect} />
      </main>
    </CenterLayout>
  );
};
