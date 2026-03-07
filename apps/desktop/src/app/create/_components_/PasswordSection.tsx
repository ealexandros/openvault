"use client";

import { PasswordStrength } from "@/components/ui/password-strength";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";
import { EyeIcon, EyeOffIcon } from "lucide-react";

type PasswordSectionProps = {
  passwordRef: React.RefObject<HTMLInputElement | null>;
  verifyPasswordRef: React.RefObject<HTMLInputElement | null>;
  passwordError: string | null;
  showPassword: boolean;
  passwordStrengthScore: number;
  toggleShowPassword: () => void;
  handlePasswordChange: () => void;
};

export const PasswordSection = ({
  passwordRef,
  verifyPasswordRef,
  passwordError,
  showPassword,
  passwordStrengthScore,
  toggleShowPassword,
  handlePasswordChange,
}: PasswordSectionProps) => (
  <div className="space-y-6">
    <div className="space-y-2">
      <div className="flex items-center justify-between px-1">
        <Label className="text-xs font-bold tracking-wider text-muted-foreground uppercase">
          Password
        </Label>
      </div>
      <div className="relative">
        <Input
          name="password"
          ref={passwordRef}
          type={showPassword ? "text" : "password"}
          placeholder="Min. 8 characters"
          onChange={handlePasswordChange}
          className={cn(
            "px-4 pr-12",
            passwordError != null ? "border-destructive/50" : "border-border",
          )}
        />
        <button
          type="button"
          onClick={toggleShowPassword}
          className="absolute top-1/2 right-4 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground">
          {showPassword ? <EyeOffIcon className="size-4" /> : <EyeIcon className="size-4" />}
        </button>
      </div>

      {passwordStrengthScore > 0 && <PasswordStrength strengthScore={passwordStrengthScore} />}

      {passwordError != null && (
        <p className="ml-1 text-xs text-destructive">{passwordError}</p>
      )}
    </div>

    <div className="space-y-2">
      <Label className="ml-1 text-xs font-bold tracking-widest text-muted-foreground uppercase">
        Verify Password
      </Label>
      <Input
        name="verifyPassword"
        ref={verifyPasswordRef}
        type="password"
        placeholder="Repeat password"
        className={cn(
          passwordError === "Passwords do not match"
            ? "border-destructive/50"
            : "border-border",
        )}
      />
      {passwordError === "Passwords do not match" && (
        <p className="ml-1 text-xs text-destructive">{passwordError}</p>
      )}
    </div>
  </div>
);
