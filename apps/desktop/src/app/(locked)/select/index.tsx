"use client";

import { GridBackground } from "@/components/layout/grid-background";
import { ActionSection } from "./_components_/ActionSection";
import { RecentVaultsList } from "./_components_/RecentVaultsList";
import { VaultHeader } from "./_components_/VaultHeader";
import { useVaultSelection } from "./useVaultSelection";

export const VaultSelectionPage = () => {
  const { selectedPath, recentVaults, select, connect } = useVaultSelection();

  return (
    <GridBackground className="p-6 selection:bg-primary/80 sm:p-12">
      <main className="mx-auto w-full max-w-2xl space-y-12">
        <VaultHeader />

        <div className="grid gap-12 lg:grid-cols-[1fr,320px]">
          <div className="space-y-8">
            <ActionSection
              selectedPath={selectedPath}
              onSelectFolder={select}
              onConnect={connect}
            />
          </div>
          <RecentVaultsList vaults={recentVaults} onConnect={connect} />
        </div>
      </main>
    </GridBackground>
  );
};
