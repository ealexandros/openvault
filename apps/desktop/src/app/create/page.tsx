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
import { useSetupVault, type SetupVaultFormValues } from "./useSetupVault";

import { Separator } from "@/components/ui/shadcn/separator";
import Link from "next/link";

const SetupVaultPage = () => {
  const {
    form,
    isEncrypting,
    passwordRef,
    verifyPasswordRef,
    passwordError,
    showPassword,
    passwordStrengthScore,
    handleSubmit,
    chooseFolder,
    setIsEncrypting,
    toggleShowPassword,
    handlePasswordChange,
  } = useSetupVault();

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

        <form onSubmit={handleSubmit} className="space-y-6">
          <form.Field name="path">
            {field => (
              <LocationSelector
                path={field.state.value}
                error={(field.state.meta.errors as unknown as string[])[0]}
                touched={Boolean(field.state.meta.isTouched)}
                chooseFolder={chooseFolder}
              />
            )}
          </form.Field>

          <form.Field name="name">
            {field => (
              <VaultNameInput
                value={field.state.value}
                error={(field.state.meta.errors as unknown as string[])[0]}
                touched={Boolean(field.state.meta.isTouched)}
                onChange={e => field.handleChange((e.target as HTMLInputElement).value)}
                onBlur={field.handleBlur}
              />
            )}
          </form.Field>

          <Separator className="my-7" />

          <PasswordSection
            passwordRef={passwordRef}
            verifyPasswordRef={verifyPasswordRef}
            passwordError={passwordError}
            showPassword={showPassword}
            passwordStrengthScore={passwordStrengthScore}
            toggleShowPassword={toggleShowPassword}
            handlePasswordChange={handlePasswordChange}
          />

          <form.Subscribe
            selector={state => [state.values.encryption, state.values.compression]}>
            {([encryption, compression]) => (
              <AdvancedSettings
                encryption={encryption as string}
                compression={compression as string}
                setFieldValue={(name, value) =>
                  form.setFieldValue(name as keyof SetupVaultFormValues, value)
                }
              />
            )}
          </form.Subscribe>

          <form.Subscribe selector={state => [state.canSubmit, state.isSubmitting]}>
            {([canSubmit, isSubmitting]) => (
              <Button
                type="submit"
                disabled={canSubmit === false || isSubmitting === true}
                className="relative h-14 w-full overflow-hidden bg-primary text-sm font-bold text-primary-foreground transition-all duration-300 hover:scale-[1.02] hover:bg-primary/90 active:scale-[0.98] disabled:opacity-30 disabled:hover:scale-100">
                <span className="relative flex items-center justify-center gap-2">
                  <ShieldCheckIcon className="size-4" />
                  Encrypt Vault
                </span>
              </Button>
            )}
          </form.Subscribe>
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
