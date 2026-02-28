"use client";

import { CenterLayout } from "@/components/layout/center";
import { Button } from "@/components/ui/shadcn/button";
import { hrefs } from "@/config/hrefs";
import { ChevronLeftIcon, ShieldCheckIcon } from "lucide-react";
import { AdvancedSettings } from "./_components_/AdvancedSettings";
import { EncryptionProgress } from "./_components_/EncryptionProgress";
import { LocationSelector } from "./_components_/LocationSelector";
import { PasswordSection } from "./_components_/PasswordSection";
import { VaultNameInput } from "./_components_/VaultNameInput";
import { useSetupVault } from "./useSetupVault";

const SetupVaultPage = () => {
  const { formik, isEncrypting, router, setIsEncrypting } = useSetupVault();

  if (isEncrypting) {
    return <EncryptionProgress onCancel={() => setIsEncrypting(false)} />;
  }

  return (
    <CenterLayout className="p-6 selection:bg-primary/30 sm:p-12">
      <main className="w-full max-w-md animate-in space-y-10 duration-500 fade-in">
        <div className="flex items-center gap-4">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => router.push(hrefs.home.get())}
            className="size-10 rounded-md border border-border/50 hover:bg-muted">
            <ChevronLeftIcon className="size-5" />
          </Button>
          <div>
            <h1 className="text-xl font-semibold tracking-tight">Create New Vault</h1>
            <p className="text-xs text-muted-foreground">
              Set up a secure encrypted container
            </p>
          </div>
        </div>

        <form onSubmit={formik.handleSubmit} className="space-y-6">
          <LocationSelector
            path={formik.values.path}
            error={formik.errors.path}
            touched={formik.touched.path}
            setFieldValue={formik.setFieldValue}
          />
          <VaultNameInput
            value={formik.values.name}
            error={formik.errors.name}
            touched={formik.touched.name}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
          <PasswordSection
            passwordValue={formik.values.password}
            verifyValue={formik.values.verifyPassword}
            passwordError={formik.errors.password}
            passwordTouched={formik.touched.password}
            verifyError={formik.errors.verifyPassword}
            verifyTouched={formik.touched.verifyPassword}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />

          <AdvancedSettings
            algorithm={formik.values.algorithm}
            setFieldValue={formik.setFieldValue}
          />

          <Button
            type="submit"
            disabled={!formik.isValid || !formik.dirty || formik.isSubmitting}
            className="group relative h-14 w-full overflow-hidden rounded-2xl bg-primary text-[15px] font-bold text-primary-foreground transition-all duration-300 hover:scale-[1.02] hover:bg-primary/90 active:scale-[0.98] disabled:opacity-30 disabled:hover:scale-100">
            <span className="relative flex items-center justify-center gap-2">
              <ShieldCheckIcon className="size-4" />
              Encrypt Vault
            </span>
          </Button>
        </form>

        <div className="flex items-start gap-3 rounded-2xl border border-primary/10 bg-primary/5 p-4">
          <ShieldCheckIcon className="h-5 w-5 shrink-0 text-primary" />
          <p className="text-[11px] leading-relaxed text-muted-foreground">
            All files within the selected folder will be encrypted using industry-standard
            AES-256 GCM. Your password is the only key to reverse this process.
          </p>
        </div>
      </main>
    </CenterLayout>
  );
};

export default SetupVaultPage;
