import { Button } from "@/components/ui/shadcn/button";
import { Checkbox } from "@/components/ui/shadcn/checkbox";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";
import { ArrowLeftIcon, EyeIcon, EyeOffIcon, Loader2Icon, ShieldIcon } from "lucide-react";

type UnlockFormProps = {
  passwordRef: React.RefObject<HTMLInputElement | null>;
  isLoading: boolean;
  rememberVault: boolean;
  hasPassword: boolean;
  showPassword: boolean;
  isButtonDisabled: boolean;
  handlePasswordChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  toggleShowPassword: () => void;
  onSubmit: (e: React.FormEvent<HTMLFormElement>) => void;
  onBack: () => void;
  setRememberVault: (value: boolean) => void;
};

export const UnlockForm = ({
  passwordRef,
  isLoading,
  rememberVault,
  hasPassword,
  showPassword,
  isButtonDisabled,
  handlePasswordChange,
  toggleShowPassword,
  onSubmit,
  onBack,
  setRememberVault,
}: UnlockFormProps) => (
  <form onSubmit={onSubmit} className="space-y-4">
    <div className="group relative">
      <div className="pointer-events-none absolute inset-y-0 left-4 z-10 flex items-center">
        <ShieldIcon
          className={cn(
            "size-4 transition-all duration-300",
            hasPassword ? "scale-110 text-primary" : "text-muted-foreground/30",
          )}
        />
      </div>
      <Input
        ref={passwordRef}
        type={showPassword ? "text" : "password"}
        placeholder="Enter vault password"
        disabled={isLoading}
        onChange={handlePasswordChange}
        autoFocus
        className="w-full px-11 text-sm! placeholder:text-muted-foreground/40"
      />
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        tabIndex={-1}
        onClick={toggleShowPassword}
        disabled={isLoading}
        className="absolute top-1/2 right-3 -translate-y-1/2 cursor-pointer text-muted-foreground/20 transition-colors hover:bg-transparent hover:text-primary disabled:pointer-events-none">
        {showPassword ? <EyeOffIcon className="size-4" /> : <EyeIcon className="size-4" />}
      </Button>
    </div>

    <div className="flex cursor-pointer items-center gap-2 px-1">
      <Checkbox
        id="remember"
        checked={rememberVault}
        onCheckedChange={checked => setRememberVault(Boolean(checked))}
        disabled={isLoading}
      />
      <Label
        htmlFor="remember"
        className="cursor-pointer text-xs font-bold tracking-wider text-muted-foreground/50 uppercase transition-colors peer-disabled:opacity-50 hover:text-muted-foreground/80">
        Remember this vault
      </Label>
    </div>

    <div className="space-y-2 pt-2">
      <Button
        type="submit"
        disabled={isButtonDisabled}
        className={cn(
          "h-13 w-full text-sm font-bold transition-all duration-300",
          !isButtonDisabled &&
            "hover:scale-[1.02] hover:shadow-[0_0_20px_rgba(var(--primary),0.3)] active:scale-[0.98]",
        )}>
        {isLoading ? (
          <>
            <Loader2Icon className="size-4 animate-spin" />
            <span className="tracking-wide">Unlocking…</span>
          </>
        ) : (
          <span className="tracking-wide">Unlock Vault</span>
        )}
      </Button>

      <Button
        type="button"
        variant="ghost"
        onClick={onBack}
        disabled={isLoading}
        className="h-13 w-full gap-2 text-xs font-bold tracking-widest text-muted-foreground/40 uppercase hover:text-foreground/70">
        <ArrowLeftIcon className="size-4" />
        <span className="mt-0.5">Back to selection</span>
      </Button>
    </div>
  </form>
);
