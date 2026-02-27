import { Checkbox } from "@/components/ui/shadcn/checkbox";
import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";
import { ArrowLeftIcon, EyeIcon, EyeOffIcon, Loader2Icon, ShieldIcon } from "lucide-react";

type UnlockFormProps = {
  password: string;
  setPassword: (value: string) => void;
  showPassword: boolean;
  toggleShowPassword: () => void;
  onSubmit: () => void;
  onBack: () => void;
  isLoading: boolean;
  rememberVault: boolean;
  setRememberVault: (value: boolean) => void;
};

export const UnlockForm = ({
  password,
  setPassword,
  showPassword,
  toggleShowPassword,
  onSubmit,
  onBack,
  isLoading,
  rememberVault,
  setRememberVault,
}: UnlockFormProps) => {
  const isButtonDisabled = !password || isLoading;

  return (
    <form
      onSubmit={e => {
        e.preventDefault();
        onSubmit();
      }}
      className="space-y-4">
      <div className="group relative">
        <div className="pointer-events-none absolute inset-y-0 left-4 flex items-center">
          <ShieldIcon
            className={cn(
              "size-4 transition-all duration-300",
              password ? "scale-110 text-primary" : "text-muted-foreground/30",
            )}
          />
        </div>
        <input
          type={showPassword ? "text" : "password"}
          placeholder="Enter vault password"
          value={password}
          onChange={e => setPassword(e.target.value)}
          disabled={isLoading}
          autoFocus
          className={cn(
            "h-13 w-full rounded-lg border bg-muted/10 pr-12 pl-11",
            "text-base font-medium placeholder:text-muted-foreground/30",
            "transition-all duration-300 outline-none",
            "border-border/40 focus:border-primary/40 focus:bg-background focus:ring-[6px] focus:ring-primary/5",
            "disabled:cursor-not-allowed disabled:opacity-50",
          )}
        />
        <button
          type="button"
          tabIndex={-1}
          onClick={toggleShowPassword}
          disabled={isLoading}
          className="absolute inset-y-0 right-3 flex cursor-pointer items-center px-1 text-muted-foreground/20 transition-colors hover:text-primary disabled:pointer-events-none disabled:cursor-not-allowed">
          {showPassword ? <EyeOffIcon className="size-4" /> : <EyeIcon className="size-4" />}
        </button>
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
          className="mt-0.5 cursor-pointer text-[10px] font-bold tracking-widest text-muted-foreground/50 uppercase transition-colors peer-disabled:opacity-50 hover:text-muted-foreground/80">
          Remember this vault
        </Label>
      </div>

      <div className="space-y-2 pt-2">
        <button
          type="submit"
          disabled={isButtonDisabled}
          className={cn(
            "relative flex h-13 w-full cursor-pointer items-center justify-center gap-2 overflow-hidden rounded-lg text-sm font-bold transition-all duration-300",
            isButtonDisabled
              ? "cursor-not-allowed border border-slate-200/20 bg-slate-200/40 text-slate-400/60"
              : "bg-primary text-primary-foreground hover:scale-[1.02] hover:shadow-[0_0_20px_rgba(var(--primary),0.3)] active:scale-[0.98]",
          )}>
          {isLoading ? (
            <>
              <Loader2Icon className="size-4 animate-spin" />
              <span className="tracking-wide">Unlocking…</span>
            </>
          ) : (
            <span className="tracking-wide">Unlock Vault</span>
          )}
        </button>

        <button
          type="button"
          onClick={onBack}
          disabled={isLoading}
          className={cn(
            "flex h-12 w-full cursor-pointer items-center justify-center gap-2 rounded-lg text-xs font-bold tracking-widest text-muted-foreground/40 uppercase transition-all duration-200 hover:bg-muted/30 hover:text-foreground/70 disabled:cursor-not-allowed disabled:opacity-50",
          )}>
          <ArrowLeftIcon className="size-3" />
          Back to selection
        </button>
      </div>
    </form>
  );
};
