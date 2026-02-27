"use client";

import { CenterLayout } from "@/components/layout/center";
import { cn } from "@/utils/cn";
import { motion } from "framer-motion";
import { ActionSection } from "./components/ActionSection";
import { RecentVaultsList } from "./components/RecentVaultsList";
import { UnlockForm } from "./components/UnlockForm";
import { VaultHeader } from "./components/VaultHeader";
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
    hasRecentActivity,
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
    <CenterLayout className="overflow-hidden p-6 selection:bg-primary/80 sm:p-12">
      <main className="mx-auto w-full max-w-xl">
        {view === "selection" ? (
          <div
            className={cn("space-y-12", !hasRecentActivity && !isLoadingVaults && "-mt-12")}>
            <VaultHeader view="selection" path="" />

            <div className="grid gap-12 lg:grid-cols-[1fr,320px]">
              <div className="space-y-8">
                <ActionSection onBrowse={handleSelect} />
              </div>
              {hasRecentActivity && !isLoadingVaults && (
                <RecentVaultsList
                  vaults={recentVaults}
                  onConnect={handleConnect}
                  onRemove={handleRemoveRecent}
                  onClear={handleClearRecent}
                  isLoading={isLoadingVaults}
                />
              )}
            </div>
          </div>
        ) : (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="mx-auto max-w-md space-y-10 py-12">
            <VaultHeader view="unlock" path={selectedVaultPath ?? ""} />

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
              <p className="mx-auto max-w-xs text-center text-[12px] leading-relaxed text-muted-foreground/40">
                Your password is never stored and is used locally to derive the encryption key.
              </p>
            </div>
          </motion.div>
        )}
      </main>
    </CenterLayout>
  );
};
