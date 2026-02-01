"use client";

import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import { EyeIcon, EyeOffIcon } from "lucide-react";
import { useEffect, useState } from "react";

// @todo-soon refactor this into a reusable component..

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

export function PasswordSection({
  passwordValue,
  verifyValue,
  passwordError,
  passwordTouched,
  verifyError,
  verifyTouched,
  onChange,
  onBlur,
}: PasswordSectionProps) {
  const [showPassword, setShowPassword] = useState(false);
  const [strength, setStrength] = useState(0);

  const getPasswordStrength = (pw: string) => {
    if (!pw) return 0;
    let s = 0;
    if (pw.length >= 8) s++;
    if (/[A-Z]/.test(pw)) s++;
    if (/[0-9]/.test(pw)) s++;
    if (/[^a-zA-Z0-9]/.test(pw)) s++;
    return s;
  };

  useEffect(() => {
    // @todo-soon fix this..
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setStrength(getPasswordStrength(passwordValue));
  }, [passwordValue]);

  return (
    <div className="space-y-4 border-t border-border/50 pt-2">
      <div className="space-y-2">
        <div className="flex items-center justify-between px-1">
          <Label className="text-[11px] font-bold tracking-widest text-muted-foreground uppercase">
            Password
          </Label>
          {passwordValue && (
            <span
              className={`text-[10px] font-bold tracking-widest uppercase ${
                strength === 1
                  ? "text-red-500"
                  : strength === 2
                    ? "text-orange-500"
                    : strength === 3
                      ? "text-yellow-500"
                      : strength === 4
                        ? "text-emerald-500"
                        : ""
              }`}>
              {strength === 1
                ? "Weak"
                : strength === 2
                  ? "Fair"
                  : strength === 3
                    ? "Good"
                    : strength === 4
                      ? "Strong"
                      : ""}
            </span>
          )}
        </div>
        <div className="relative">
          <Input
            name="password"
            type={showPassword ? "text" : "password"}
            placeholder="Min. 8 characters"
            value={passwordValue}
            onChange={onChange}
            onBlur={onBlur}
            className={`h-12 rounded-2xl bg-muted/30 px-4 pr-12 focus:ring-primary/20 ${
              passwordTouched === true && passwordError != null
                ? "border-red-500/50"
                : "border-border"
            }`}
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute top-1/2 right-4 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground">
            {showPassword ? (
              <EyeOffIcon className="h-4 w-4" />
            ) : (
              <EyeIcon className="h-4 w-4" />
            )}
          </button>
        </div>

        {/* Strength Meter */}
        <div className="flex h-1 gap-1.5 px-1">
          {[1, 2, 3, 4].map(i => (
            <div
              key={i}
              className={`flex-1 rounded-full transition-all duration-500 ${
                i <= strength
                  ? strength === 1
                    ? "bg-red-500"
                    : strength === 2
                      ? "bg-orange-500"
                      : strength === 3
                        ? "bg-yellow-500"
                        : "bg-emerald-500"
                  : "bg-muted/30"
              }`}
            />
          ))}
        </div>
        {passwordTouched === true && passwordError != null && (
          <p className="ml-1 text-[10px] font-medium text-red-500">{passwordError}</p>
        )}
      </div>

      <div className="space-y-2">
        <Label className="ml-1 text-[11px] font-bold tracking-widest text-muted-foreground uppercase">
          Verify Password
        </Label>
        <Input
          name="verifyPassword"
          type="password"
          placeholder="Repeat password"
          value={verifyValue}
          onChange={onChange}
          onBlur={onBlur}
          className={`h-12 rounded-2xl bg-muted/30 px-4 focus:ring-primary/20 ${
            verifyTouched === true && verifyError != null
              ? "border-red-500/50"
              : "border-border"
          }`}
        />
        {verifyTouched === true && verifyError != null && (
          <p className="ml-1 text-[10px] font-medium text-red-500">{verifyError}</p>
        )}
      </div>
    </div>
  );
}
