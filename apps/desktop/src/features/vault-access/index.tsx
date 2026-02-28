"use client";

import { CenterLayout } from "@/components/layout/center";
import { ActionSection } from "./components/ActionSection";
import { RecentVaultsList } from "./components/RecentVaultsList";
import { SelectionVaultHeader } from "./components/SelectionVaultHeader";
import { UnlockForm } from "./components/UnlockForm";
import { UnlockedVaultHeader } from "./components/UnlockedVaultHeader";
import { useVaultAccess } from "./hooks/useVaultAccess";

export const VaultAccessScreen = () => {
  const {
    view,
    recentVaults,
    selectedVaultPath,
    isLoadingVaults,
    isUnlocking: isProcessing,
    password,
    rememberVault,
    showPassword,
    setPassword,
    toggleShowPassword,
    handleSelect,
    handleConnect,
    handleUnlock,
    handleBack,
    handleRemoveRecent,
    handleClearRecent,
    setRememberVault,
  } = useVaultAccess();

  return (
    <CenterLayout className="overflow-hidden p-6 sm:p-12">
      <main className="w-full max-w-xl">
        {view === "selection" ? (
          <div className="space-y-12">
            <SelectionVaultHeader />
            <ActionSection onBrowse={handleSelect} />
            <RecentVaultsList
              vaults={recentVaults}
              onConnect={handleConnect}
              onRemove={handleRemoveRecent}
              onClear={handleClearRecent}
              isLoading={isLoadingVaults}
            />
          </div>
        ) : (
          <div className="mx-auto max-w-md animate-in space-y-10 duration-300 fade-in">
            <UnlockedVaultHeader path={selectedVaultPath ?? ""} />
            <div className="space-y-8">
              <UnlockForm
                password={password}
                setPassword={setPassword}
                showPassword={showPassword}
                toggleShowPassword={toggleShowPassword}
                onSubmit={handleUnlock}
                onBack={handleBack}
                isLoading={isProcessing}
                rememberVault={rememberVault}
                setRememberVault={setRememberVault}
              />
              <p className="mx-auto max-w-xs text-center text-xs leading-relaxed text-muted-foreground/50">
                Your password is never stored and is used locally to derive the encryption key.
              </p>
            </div>
          </div>
        )}
      </main>
    </CenterLayout>
  );
};
