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

import { Separator } from "@/components/ui/shadcn/separator";
import { cn } from "@/utils/cn";
import Link from "next/link";

const SetupVaultPage = () => {
  const { formik, isEncrypting, chooseFolder, setIsEncrypting } = useSetupVault();

  if (isEncrypting) {
    return <EncryptionProgress onCancel={() => setIsEncrypting(false)} />;
  }

  return (
    <CenterLayout>
      <main className="w-full max-w-lg animate-in space-y-10 duration-500 fade-in">
        <header className="flex items-center gap-4">
          <Button variant="outline" size="icon" className="size-10" asChild>
            <Link href={hrefs.home.get()}>
              <ChevronLeftIcon className="size-5" />
            </Link>
          </Button>
          <div className="space-y-1">
            <h1 className="text-2xl font-semibold tracking-tight">Create a Vault</h1>
            <p className="text-base text-muted-foreground">
              Set up a secure encrypted container
            </p>
          </div>
        </header>

        <form onSubmit={formik.handleSubmit} className="space-y-6">
          <LocationSelector
            path={formik.values.path}
            error={formik.errors.path}
            touched={formik.touched.path}
            chooseFolder={chooseFolder}
          />
          <VaultNameInput
            value={formik.values.name}
            error={formik.errors.name}
            touched={formik.touched.name}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
          <Separator className="my-7" />
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
            className={cn(
              "group relative h-14 w-full overflow-hidden rounded-2xl bg-primary text-sm font-bold text-primary-foreground transition-all duration-300 hover:scale-[1.02] hover:bg-primary/90 active:scale-[0.98] disabled:opacity-30 disabled:hover:scale-100",
            )}>
            <span className="relative flex items-center justify-center gap-2">
              <ShieldCheckIcon className="size-4" />
              Encrypt Vault
            </span>
          </Button>
        </form>

        <div className="flex items-start gap-3 rounded-2xl border border-primary/10 bg-primary/5 p-4">
          <ShieldCheckIcon className="size-5 shrink-0 text-primary" />
          <p className="text-sm leading-relaxed text-muted-foreground">
            All files within the selected folder will be encrypted using industry-standards.
            Your password is the only key to reverse this process.
          </p>
        </div>
      </main>
    </CenterLayout>
  );
};

export default SetupVaultPage;
