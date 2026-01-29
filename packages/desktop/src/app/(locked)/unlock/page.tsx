"use client";

import { CenterLayout } from "@/components/layout/center";
import { hrefs } from "@/config/hrefs";
import { UnlockFooter } from "./_components_/UnlockFooter";
import { UnlockForm } from "./_components_/UnlockForm";
import { UnlockHeader } from "./_components_/UnlockHeader";
import { useVaultUnlock } from "./useVaultUnlock";

const UnlockPage = () => {
  const {
    selectedPath,
    password,
    setPassword,
    showPassword,
    toggleShowPassword,
    handleBack,
    handleProcessVault,
    router,
  } = useVaultUnlock();

  if (selectedPath == null) {
    router.push(hrefs.home.get());
    return null;
  }

  return (
    <CenterLayout className="p-6 selection:bg-primary/30 sm:p-12">
      <main className="z-10 w-full max-w-lg space-y-6">
        <UnlockHeader onBack={handleBack} selectedPath={selectedPath} />
        <UnlockForm
          password={password}
          setPassword={setPassword}
          showPassword={showPassword}
          toggleShowPassword={toggleShowPassword}
          onSubmit={handleProcessVault}
        />
        <UnlockFooter />
      </main>
    </CenterLayout>
  );
};

export default UnlockPage;
