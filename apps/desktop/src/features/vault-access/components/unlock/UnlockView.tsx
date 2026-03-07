import { useUnlockVault } from "../../hooks/useUnlockVault";
import { UnlockedVaultHeader } from "./UnlockedVaultHeader";
import { UnlockForm } from "./UnlockForm";

type UnlockViewProps = {
  selectedVaultPath: string | null;
  onBack: () => void;
};

export const UnlockView = ({ selectedVaultPath, onBack }: UnlockViewProps) => {
  const {
    passwordRef,
    isUnlocking,
    rememberVault,
    hasPassword,
    showPassword,
    isButtonDisabled,
    handlePasswordChange,
    toggleShowPassword,
    handleSubmit,
    setRememberVault,
  } = useUnlockVault(selectedVaultPath);

  return (
    <div className="mx-auto max-w-md animate-in space-y-10 duration-300 fade-in">
      <UnlockedVaultHeader path={selectedVaultPath ?? ""} />
      <div className="space-y-8">
        <UnlockForm
          passwordRef={passwordRef}
          hasPassword={hasPassword}
          handlePasswordChange={handlePasswordChange}
          showPassword={showPassword}
          toggleShowPassword={toggleShowPassword}
          onSubmit={handleSubmit}
          onBack={onBack}
          isLoading={isUnlocking}
          rememberVault={rememberVault}
          setRememberVault={setRememberVault}
          isButtonDisabled={isButtonDisabled}
        />
        <p className="mx-auto max-w-xs text-center text-xs leading-relaxed text-muted-foreground/50">
          Your password is never stored and is used locally to derive the encryption key.
        </p>
      </div>
    </div>
  );
};
