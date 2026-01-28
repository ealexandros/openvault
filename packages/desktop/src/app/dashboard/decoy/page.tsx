"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { cn } from "@/utils/cn";
import {
  EyeIcon,
  EyeOffIcon,
  FileTextIcon,
  LockIcon,
  PlusIcon,
  ShieldAlertIcon,
} from "lucide-react";
import { useState } from "react";

const DecoyPage = () => {
  const [decoyState, setDecoyState] = useState<"setup" | "unlocked">("setup");
  const [decoyPassword, setDecoyPassword] = useState("");
  const [showDecoyPassword, setShowDecoyPassword] = useState(false);

  const getPasswordStrength = (pw: string) => {
    if (!pw) return 0;
    let strength = 0;
    if (pw.length > 8) strength++;
    if (/[A-Z]/.test(pw)) strength++;
    if (/[0-9]/.test(pw)) strength++;
    if (/[^A-Za-z0-9]/.test(pw)) strength++;
    return strength;
  };

  const strength = getPasswordStrength(decoyPassword);

  return (
    <div className="mx-auto max-w-5xl space-y-8">
      {decoyState === "setup" ? (
        <div className="mx-auto max-w-md space-y-8 py-12">
          <div className="space-y-2 text-center">
            <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-2xl border border-amber-500/20 bg-amber-500/10">
              <ShieldAlertIcon className="size-8 text-amber-500" />
            </div>
            <h3 className="text-xl font-semibold tracking-tight">Setup Decoy Vault</h3>
            <p className="text-sm text-muted-foreground">
              Create a fake vault to protect your real data under duress.
            </p>
          </div>

          <div className="space-y-4">
            <div className="space-y-2">
              <div className="flex items-end justify-between px-1">
                <label className="text-[10px] font-bold tracking-widest text-muted-foreground uppercase">
                  Decoy Password
                </label>
                <span
                  className={cn(
                    "text-[10px] font-bold tracking-widest uppercase",
                    strength <= 1
                      ? "text-red-500"
                      : strength === 2
                        ? "text-amber-500"
                        : "text-emerald-500",
                  )}>
                  {strength === 0
                    ? ""
                    : strength <= 1
                      ? "Weak"
                      : strength <= 3
                        ? "Medium"
                        : "Strong"}
                </span>
              </div>
              <div className="relative">
                <Input
                  type={showDecoyPassword ? "text" : "password"}
                  placeholder="Min 8 characters..."
                  value={decoyPassword}
                  onChange={e => setDecoyPassword(e.target.value)}
                  className="h-11 rounded-xl border-border/50 bg-muted/20 pr-10 focus:ring-amber-500/20"
                />
                <button
                  type="button"
                  onClick={() => setShowDecoyPassword(!showDecoyPassword)}
                  className="absolute top-1/2 right-3 -translate-y-1/2 text-muted-foreground transition-colors hover:text-foreground">
                  {showDecoyPassword ? (
                    <EyeOffIcon className="size-4" />
                  ) : (
                    <EyeIcon className="size-4" />
                  )}
                </button>
              </div>
              <div className="flex h-1 gap-1 px-0.5">
                {[1, 2, 3, 4].map(i => (
                  <div
                    key={i}
                    className={cn(
                      "flex-1 rounded-full transition-all duration-500",
                      i <= strength
                        ? strength <= 1
                          ? "bg-red-500"
                          : strength === 2
                            ? "bg-amber-500"
                            : "bg-emerald-500"
                        : "bg-muted/30",
                    )}
                  />
                ))}
              </div>
            </div>

            <Button
              disabled={decoyPassword.length < 8}
              onClick={() => setDecoyState("unlocked")}
              className="h-11 w-full rounded-xl bg-amber-500 font-medium text-white shadow-lg shadow-amber-500/10 transition-all hover:bg-amber-600">
              Create Decoy Vault
            </Button>
          </div>

          <div className="flex items-start gap-3 rounded-xl border border-border/50 bg-muted/10 p-4">
            <div className="shrink-0 rounded-lg bg-amber-500/10 p-1.5 text-amber-500">
              <LockIcon className="size-3.5" />
            </div>
            <p className="text-[11px] leading-relaxed text-muted-foreground">
              The decoy password should be completely different from your main password. It
              opens a separate, harmless directory you specify later.
            </p>
          </div>
        </div>
      ) : (
        <div className="animate-in space-y-8 duration-500 fade-in slide-in-from-bottom-2">
          <div className="flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium">Decoy Files</h3>
              <p className="text-xs text-muted-foreground">
                Files visible when decoy password is used
              </p>
            </div>
            <div className="flex gap-2">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setDecoyState("setup");
                  setDecoyPassword("");
                }}
                className="h-8 text-xs text-muted-foreground hover:text-foreground">
                Reset Decoy
              </Button>
              <Button
                size="sm"
                className="h-8 rounded-lg bg-amber-500 px-4 text-xs font-medium text-white hover:bg-amber-600">
                <PlusIcon className="mr-2 size-3.5" />
                Add Decoy File
              </Button>
            </div>
          </div>

          <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            {[1, 2, 3].map(i => (
              <div
                key={i}
                className="group flex cursor-pointer items-center gap-4 rounded-xl border border-border/50 bg-muted/20 p-4 transition-all hover:border-amber-500/30">
                <div className="rounded-lg border border-border bg-background p-2">
                  <FileTextIcon className="size-5 text-muted-foreground transition-colors group-hover:text-amber-500" />
                </div>
                <div className="min-w-0">
                  <p className="truncate text-sm font-medium">public_memo_{i}.docx</p>
                  <p className="text-[10px] text-muted-foreground">General Document</p>
                </div>
              </div>
            ))}
          </div>

          <div className="flex flex-col items-center justify-center space-y-3 rounded-2xl border border-dashed border-border/50 p-8 text-center">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-muted/20 text-muted-foreground">
              <PlusIcon className="size-6" />
            </div>
            <div className="space-y-1">
              <p className="text-sm font-medium">Drop files to add to decoy</p>
              <p className="text-xs text-muted-foreground">
                These files will be stored unencrypted in the decoy path.
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default DecoyPage;
