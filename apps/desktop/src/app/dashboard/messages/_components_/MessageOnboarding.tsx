"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { Label } from "@/components/ui/shadcn/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/shadcn/select";
import { cn } from "@/utils/cn";
import { AnimatePresence, motion } from "framer-motion";
import { ArrowRight, ShieldCheck, User, UserPlus, Users } from "lucide-react";
import { useState } from "react";
import { useKeyListeners } from "@/hooks/useKeyListeners";
import { KeyCode } from "@/config/keycodes";

type MessageOnboardingProps = {
  currentUserName: string;
  onComplete: (data: { name: string; rotationMonths: number }) => void;
  openImportPicker: () => void;
};

const steps = ["intro", "identity", "peers", "ready"] as const;
type Step = (typeof steps)[number];

const Progress = ({ currentStepIndex }: { currentStepIndex: number }) => (
  <div className="flex justify-center gap-1.5 py-4">
    {steps.map((s, i) => (
      <div
        key={s}
        className={cn(
          "h-1 rounded-full transition-all duration-500",
          i <= currentStepIndex ? "w-8 bg-primary" : "w-1.5 bg-muted-foreground/20",
        )}
      />
    ))}
  </div>
);

export const MessageOnboarding = ({
  currentUserName,
  onComplete,
  openImportPicker,
}: MessageOnboardingProps) => {
  const [step, setStep] = useState<Step>("intro");
  const [displayName, setDisplayName] = useState(currentUserName);
  const [rotationMonths, setRotationMonths] = useState(12);

  const currentStepIndex = steps.indexOf(step);

  const isNextDisabled = step === "identity" && !displayName.trim();

  const handleNext = () => {
    if (isNextDisabled) return;
    if (step === "intro") setStep("identity");
    else if (step === "identity") setStep("peers");
    else if (step === "peers") setStep("ready");
    else onComplete({ name: displayName, rotationMonths });
  };

  useKeyListeners({
    [KeyCode.Enter]: handleNext,
  });

  const handleBack = () => {
    if (step === "identity") setStep("intro");
    else if (step === "peers") setStep("identity");
    else if (step === "ready") setStep("peers");
  };

  return (
    <div className="flex h-full w-full items-center justify-center bg-linear-to-b from-background to-muted/20 p-6">
      <div className="w-full max-w-lg space-y-8">
        <AnimatePresence mode="popLayout" initial={false}>
          {step === "intro" && (
            <motion.div
              key="intro"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.3, ease: "easeOut" }}
              className="space-y-8 text-center">
              <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
                <ShieldCheck className="h-10 w-10" />
              </div>
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tight">Secure Messaging</h2>
                <p className="text-muted-foreground">
                  Experience end-to-end encrypted communication. Share credentials, messages,
                  and digital signatures with ultimate privacy.
                </p>
              </div>
            </motion.div>
          )}

          {step === "identity" && (
            <motion.div
              key="identity"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.3, ease: "easeOut" }}
              className="space-y-8">
              <div className="space-y-2">
                <h3 className="text-3xl font-bold tracking-tight">Set Your Identity</h3>
                <p className="text-muted-foreground">
                  This identity will be visible to your trusted peers.
                </p>
              </div>

              <div className="space-y-6">
                <div className="space-y-2">
                  <Label
                    htmlFor="displayName"
                    className="text-xs font-bold tracking-widest text-muted-foreground/60 uppercase">
                    Display Name
                  </Label>
                  <Input
                    id="displayName"
                    value={displayName}
                    onChange={e => setDisplayName(e.target.value)}
                    placeholder="Alice..."
                    className="h-12 w-full bg-background text-base"
                  />
                </div>

                <div className="space-y-2">
                  <Label className="text-xs font-bold tracking-widest text-muted-foreground/60 uppercase">
                    Key Rotation Policy
                  </Label>
                  <Select
                    value={rotationMonths.toString()}
                    onValueChange={v => setRotationMonths(parseInt(v))}>
                    <SelectTrigger className="h-12 w-full bg-background text-base">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="3">Every 3 months (Highly Secure)</SelectItem>
                      <SelectItem value="6">Every 6 months (Balanced)</SelectItem>
                      <SelectItem value="12">Every 12 months (Standard)</SelectItem>
                      <SelectItem value="24">Every 24 months (Long Term)</SelectItem>
                    </SelectContent>
                  </Select>
                  <p className="text-xs leading-relaxed text-muted-foreground/70">
                    * Automated key rotation helps minimize the impact of long-term credential
                    leakage.
                  </p>
                </div>
              </div>
            </motion.div>
          )}

          {step === "peers" && (
            <motion.div
              key="peers"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.3, ease: "easeOut" }}
              className="space-y-8">
              <div className="space-y-4 text-center">
                <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
                  <Users className="h-10 w-10" />
                </div>
                <div className="space-y-2">
                  <h3 className="text-3xl font-bold tracking-tight">Connect with Peers</h3>
                  <p className="mx-auto max-w-[320px] text-muted-foreground">
                    Import public keys from your friends to start exchange messages securely.
                  </p>
                </div>
              </div>

              <Button
                variant="outline"
                onClick={openImportPicker}
                className="group h-20 w-full justify-start gap-4 rounded-2xl border-dashed px-6 hover:border-primary/50 hover:bg-primary/5">
                <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 text-primary transition-colors group-hover:bg-primary group-hover:text-primary-foreground">
                  <UserPlus className="h-6 w-6" />
                </div>
                <div className="text-left">
                  <p className="text-sm font-semibold">Import peer identities</p>
                  <p className="text-xs text-muted-foreground">Choose .ovp profiles</p>
                </div>
              </Button>
            </motion.div>
          )}

          {step === "ready" && (
            <motion.div
              key="ready"
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.3, ease: "easeOut" }}
              className="space-y-8 text-center">
              <div className="mx-auto flex h-20 w-20 items-center justify-center rounded-3xl bg-primary text-primary-foreground">
                <User className="h-10 w-10" />
              </div>

              <div className="space-y-2 text-center">
                <h3 className="text-3xl font-bold tracking-tight">Everything is Ready</h3>
                <p className="text-muted-foreground">
                  Your cryptographic workspace has been initialized. All communication at this
                  point will be end-to-end encrypted.
                </p>
              </div>

              <div className="divide-y divide-border/50 rounded-lg border border-border/50 bg-muted/20 text-sm">
                <div className="flex items-center justify-between p-5 text-left">
                  <span className="font-medium text-muted-foreground">Identity</span>
                  <span className="font-semibold">{displayName}</span>
                </div>
                <div className="flex items-center justify-between p-5 text-left">
                  <span className="font-medium text-muted-foreground">Rotation</span>
                  <span className="font-semibold">Every {rotationMonths} months</span>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        <div className="space-y-4">
          <Progress currentStepIndex={currentStepIndex} />
          <div className="flex flex-col gap-2">
            {step === "intro" && (
              <Button
                size="lg"
                onClick={handleNext}
                className="h-12 w-full gap-2 text-sm font-semibold">
                Start Setup
                <ArrowRight className="h-4 w-4" />
              </Button>
            )}
            {step === "identity" && (
              <>
                <Button
                  onClick={handleNext}
                  disabled={!displayName.trim()}
                  className="h-12 w-full gap-2 text-sm font-semibold">
                  Next Step
                  <ArrowRight className="size-4" />
                </Button>
                <Button
                  variant="ghost"
                  onClick={handleBack}
                  className="h-12 w-full text-sm text-muted-foreground">
                  Back
                </Button>
              </>
            )}
            {step === "peers" && (
              <>
                <Button onClick={handleNext} className="h-12 w-full text-sm font-semibold">
                  Skip for now
                </Button>
                <Button
                  variant="ghost"
                  onClick={handleBack}
                  className="h-12 w-full text-sm text-muted-foreground">
                  Back
                </Button>
              </>
            )}
            {step === "ready" && (
              <Button
                size="lg"
                onClick={() => onComplete({ name: displayName, rotationMonths })}
                className="group h-12 w-full gap-1 text-sm font-semibold">
                Enter Workspace
                <ArrowRight className="size-4 transition-transform group-hover:translate-x-0.5" />
              </Button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
