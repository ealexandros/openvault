"use client";

import { PasswordStrength } from "@/components/ui/password-strength";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { cn } from "@/utils/cn";
import { EyeIcon, EyeOffIcon } from "lucide-react";
import { useState } from "react";

type PasswordSectionProps = {
  passwordValue: string;
  verifyValue: string;
  passwordError?: string;
  passwordTouched?: boolean;
  verifyError?: string;
  verifyTouched?: boolean;
  onChange: (e: React.ChangeEvent<unknown>) => void;
  onBlur: (e: unknown) => void;
};

export const PasswordSection = ({
  passwordValue,
  verifyValue,
  passwordError,
  passwordTouched,
  verifyError,
  verifyTouched,
  onChange,
  onBlur,
}: PasswordSectionProps) => {
  const [showPassword, setShowPassword] = useState(false);

  return (
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
            type={showPassword ? "text" : "password"}
            placeholder="Min. 8 characters"
            value={passwordValue}
            onChange={onChange}
            onBlur={onBlur}
            className={cn(
              "px-4 pr-12",
              passwordTouched === true && passwordError != null
                ? "border-destructive/50"
                : "border-border",
            )}
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute top-1/2 right-4 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground">
            {showPassword ? <EyeOffIcon className="size-4" /> : <EyeIcon className="size-4" />}
          </button>
        </div>

        {passwordValue && <PasswordStrength password={passwordValue} />}

        {passwordTouched === true && passwordError != null && (
          <p className="ml-1 text-xs text-destructive">{passwordError}</p>
        )}
      </div>

      <div className="space-y-2">
        <Label className="ml-1 text-xs font-bold tracking-widest text-muted-foreground uppercase">
          Verify Password
        </Label>
        <Input
          name="verifyPassword"
          type="password"
          placeholder="Repeat password"
          value={verifyValue}
          onChange={onChange}
          onBlur={onBlur}
          className={cn(
            verifyTouched === true && verifyError != null
              ? "border-destructive/50"
              : "border-border",
          )}
        />
        {verifyTouched === true && verifyError != null && (
          <p className="ml-1 text-xs text-destructive">{verifyError}</p>
        )}
      </div>
    </div>
  );
};
